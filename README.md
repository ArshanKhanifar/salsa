# Salsa Monorepo

This is a Rust monorepo containing shared libraries and CLI tools.

## Quick Install

Install the `parallel` tool with a single command:
```bash
curl -L https://raw.githubusercontent.com/arshankhanifar/salsa/main/install.sh | bash
```

This will automatically detect your system architecture and install the appropriate binary.

## Project Structure

- `crates/common`: Shared library containing common functionality
- `crates/cli`: Example CLI application using the common library
- `crates/parallel`: Tool for running commands in parallel with different arguments

## Getting Started

1. Build all projects:
```bash
cargo build
```

2. Run tests:
```bash
cargo test
```

3. Run the CLI:
```bash
cargo run -p cli -- --first 10 --second 20
```

## Installing the Parallel Tool

### Quick Install (Same Architecture)
If you're installing on the same architecture as your build machine:
```bash
make install-parallel
```
This will build and install the binary to `/usr/local/bin/parallel`.

### Cross-Platform Build from macOS
To build the parallel tool for both arm64 and amd64 architectures from macOS:

1. The build system will automatically install:
   - cargo-zigbuild: A Rust cross-compilation tool using Zig
   - zig: A native cross-compilation toolchain

2. Build for both architectures:
```bash
make build-parallel-all
```
This will create:
- `dist/parallel-arm64`: ARM64 binary
- `dist/parallel-amd64`: AMD64 binary

3. Install on target machine:
```bash
# The binary name will match your architecture (arm64 or amd64)
sudo cp parallel-$(uname -m) /usr/local/bin/parallel
chmod +x /usr/local/bin/parallel
```

## Adding New Crates

1. Create a new directory under `crates/`
2. Add the new crate to the workspace members in the root `Cargo.toml`
3. Reference the common library in your new crate's `Cargo.toml` if needed:
```toml
[dependencies]
common = { path = "../common" }
```

## Using the Parallel Tool

Run commands in parallel with different arguments:
```bash
parallel --cmd "your_command" --args "arg1 arg2 arg3"
```

Example:
```bash
# Run a command that sometimes fails with different arguments
parallel --cmd "errorsSometimes" --args "true false false true"
```

Features:
- Executes commands in parallel
- Shows real-time output from each process
- Color-coded output for easy identification
- Exits immediately if any process fails
- Shows execution time statistics
