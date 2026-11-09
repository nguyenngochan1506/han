use clap::Parser;

/// Quản lý SSH host, tunel và công cụ từ terminal
#[derive(Parser)]
#[command(version)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
}
