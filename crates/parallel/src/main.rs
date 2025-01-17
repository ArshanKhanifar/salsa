use anyhow::{Context, Result};
use async_process::{Child, Command, Stdio};
use clap::Parser;
use colored::*;
use futures::StreamExt;
use std::time::Instant;
use tokio::signal::ctrl_c;
use tokio::sync::mpsc;
use std::sync::Arc;
use futures::AsyncBufReadExt;
use futures::io::BufReader;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Run commands in parallel with different arguments",
    long_about = None
)]
struct Args {
    #[arg(long)]
    args: String,

    #[arg(long)]
    cmd: String,
}

struct CommandHandle {
    cmd_info: String,
    full_cmd: String,
    child: Child,
    any_failed: Arc<AtomicBool>,
}

async fn run_command_with_arg(cmd: &str, arg: &str, index: usize, any_failed: Arc<AtomicBool>) -> Result<CommandHandle> {
    let wrapped_cmd = format!(
        r#"bash -c 'source ~/.bashrc 2> /dev/null; {} {}' -- {}"#,
        cmd, 
        arg,
        arg
    );
    
    // Get the first word of the command as the name
    let cmd_name = cmd.split_whitespace().next().unwrap_or(cmd);
    let full_cmd = format!("{}", cmd);
    
    let child = Command::new("bash")
        .arg("-c")
        .arg(&wrapped_cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .with_context(|| format!("Failed to execute command: {} with arg: {}", cmd, arg))?;

    Ok(CommandHandle {
        cmd_info: format!("{}-{}", cmd_name, index),
        full_cmd,
        child,
        any_failed,
    })
}

async fn handle_ctrl_c(tx: mpsc::Sender<()>) {
    if let Ok(()) = ctrl_c().await {
        println!("\n{}", "Received Ctrl+C, terminating all processes...".yellow());
        let _ = tx.send(()).await;
    }
}

async fn stream_output(mut handle: CommandHandle) {
    let stdout = handle.child.stdout.take();
    let stderr = handle.child.stderr.take();
    let cmd_info = handle.cmd_info.clone();
    let cmd_info2 = cmd_info.clone();
    let cmd_info3 = cmd_info.clone();
    let full_cmd = handle.full_cmd.clone();
    let any_failed = handle.any_failed.clone();

    let stdout_future = async move {
        if let Some(stdout) = stdout {
            let mut reader = BufReader::new(stdout).lines();
            while let Some(line_result) = reader.next().await {
                match line_result {
                    Ok(line) => println!("{} - {}", cmd_info.bright_blue(), line),
                    Err(e) => eprintln!("{} Error reading stdout: {}", cmd_info.bright_blue(), e.to_string().red()),
                }
            }
        }
    };

    let stderr_future = async move {
        if let Some(stderr) = stderr {
            let mut reader = BufReader::new(stderr).lines();
            while let Some(line_result) = reader.next().await {
                match line_result {
                    Ok(line) => eprintln!("{} - {}", cmd_info2.bright_blue(), line),
                    Err(e) => eprintln!("{} Error reading stderr: {}", cmd_info2.bright_blue(), e.to_string().red()),
                }
            }
        }
    };

    // Run both stdout and stderr handling concurrently
    futures::join!(stdout_future, stderr_future);

    // Wait for the process to finish
    match handle.child.status().await {
        Ok(status) => {
            if !status.success() && status.code() != Some(130) { // Don't show error for Ctrl+C
                eprintln!("{} - Process exited with status: {} (command: {})", 
                    cmd_info3.bright_blue(), 
                    status.to_string().red(),
                    full_cmd.yellow()
                );
                any_failed.store(true, Ordering::SeqCst);
            }
        }
        Err(e) => {
            eprintln!("{} Error waiting for process: {} (command: {})", 
                cmd_info3.bright_blue(), 
                e.to_string().red(),
                full_cmd.yellow()
            );
            any_failed.store(true, Ordering::SeqCst);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let arguments: Vec<_> = args.args
        .split_whitespace()
        .collect();

    if arguments.is_empty() {
        println!("{}", "No arguments provided".yellow());
        return Ok(());
    }

    println!("{}", "Starting parallel execution...".green());
    let start = Instant::now();

    let mut handles: Vec<CommandHandle> = vec![];
    
    // Create a shared flag to indicate if any process has failed
    let any_failed = Arc::new(AtomicBool::new(false));

    // Spawn all processes
    for (index, arg) in arguments.iter().enumerate() {
        match run_command_with_arg(&args.cmd, arg, index, any_failed.clone()).await {
            Ok(handle) => handles.push(handle),
            Err(e) => {
                eprintln!("{}: {}", "Error spawning process".bold().red(), e);
                // Kill any already-started processes before exiting
                for mut handle in handles {
                    let _ = handle.child.kill();
                }
                return Err(e);
            }
        }
    }

    println!("\n{}", "Starting output streams:".bold());
    println!("{}", "=".repeat(50));

    let mut tasks = Vec::new();
    for handle in handles {
        tasks.push(tokio::spawn(async move {
            stream_output(handle).await;
        }));
    }

    // Wait for all tasks to complete or for any to fail
    loop {
        if any_failed.load(Ordering::SeqCst) {
            // Kill all remaining processes
            for task in tasks {
                task.abort();
            }
            println!("\n{}", "One process failed, terminating all processes...".red());
            break;
        }

        // Check if all tasks are done
        let mut all_done = true;
        for task in &tasks {
            if !task.is_finished() {
                all_done = false;
                break;
            }
        }
        if all_done {
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    let duration = start.elapsed();
    println!("\nTotal execution time: {:?}", duration);

    // If any process failed, return an error
    if any_failed.load(Ordering::SeqCst) {
        std::process::exit(1);
    }

    Ok(())
}
