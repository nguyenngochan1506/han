mod cli;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Ssh { host }) => println!("TODO: ssh tới {host}"),
        None => println!("TODO: mở TUI"),
    }
}
