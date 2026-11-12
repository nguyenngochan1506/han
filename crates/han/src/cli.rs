use clap::{Parser, Subcommand};

/// Quản lý SSH host, tunnel và công cụ từ terminal
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Kết nối SSH tới một host
    Ssh {
        /// Tên host trong ~/.ssh/config
        host: String,
    },
    /// Quản lý danh sách host
    Host {
        #[command(subcommand)]
        action: HostAction,
    },
}

#[derive(Subcommand)]
pub enum HostAction {
    ///Liệt kê ra các host trong ~/.ssh/config
    Ls,
}
