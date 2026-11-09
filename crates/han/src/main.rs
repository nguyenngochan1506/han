mod cli;

use clap::Parser;
use cli::{Cli, Command};
use std::os::unix::process::CommandExt;
use std::process;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Ssh { host }) => {
            let err = process::Command::new("ssh").arg(host).exec();
            eprintln!("han: Không chạy được ssh: {err}");
            process::exit(1);
        }
        None => println!("TODO: mở TUI"),
    }
}
