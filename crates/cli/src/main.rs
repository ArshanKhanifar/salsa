use clap::Parser;
use common::{add_numbers, Config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    first: i32,

    #[arg(short, long)]
    second: i32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let result = add_numbers(args.first, args.second);
    println!("Result: {}", result);

    // Example of using the Config struct from common
    let config = Config {
        name: "example".to_string(),
        value: result,
    };
    println!("Config: {:?}", config);
}
