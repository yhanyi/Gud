use crate::utils::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "gud")]
#[command(about = "A simple version control system!", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
