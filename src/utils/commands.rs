use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialises a new repository")]
    Init,
    #[command(about = "Add files to the staging area")]
    Add {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    #[command(about = "Commit changes to the repository")]
    Commit {
        #[arg(short = 'm', long)]
        message: String,
    },
    #[command(about = "Show the status of the repository")]
    Status,
}
