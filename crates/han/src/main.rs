mod cli;

use clap::Parser;
use cli::{Cli, Command, HostAction};
use han_core::ssh_config;
use std::os::unix::process::CommandExt;
use std::process;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Host {
            action: HostAction::Ls,
        }) => {
            let path = std::env::home_dir().unwrap_or_default().join(".ssh/config");
            match ssh_config::load_hosts(&path) {
                Ok(hosts) => {
                    for h in hosts {
                        println!("{h}")
                    }
                }
                Err(err) => {
                    eprintln!("Không đọc được {}: {err}", path.display());
                    process::exit(1);
                }
            }
        }
        Some(Command::Ssh { host }) => {
            let err = process::Command::new("ssh").arg(host).exec();
            eprintln!("han: Không chạy được ssh: {err}");
            process::exit(1);
        }
        None => println!("TODO: mở TUI"),
    }
}
