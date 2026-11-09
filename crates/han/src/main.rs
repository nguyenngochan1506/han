use clap::{Parser, Subcommand};

/// Quản lý SSH host, tunnel và công cụ từ terminal
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Kết nối SSH tới một host
    Ssh {
        /// Tên host trong ~/.ssh/config
        host: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Ssh { host }) => println!("TODO: ssh tới {host}"),
        None => println!("TODO: mở TUI"),
    }
}
