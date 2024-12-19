mod utils;

use crate::utils::cli::Cli;
use crate::utils::commands::Commands;
use clap::Parser;
use utils::repo::Repository;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let dir = std::env::current_dir()?;

    match cli.command {
        Commands::Init => {
            Repository::init(&dir)?;
            println!("Initialised empty Gud repository in {:?}", dir);
        }

        Commands::Add { path } => {
            let mut repo = Repository::load(&dir)?;
            let path = if path.is_relative() {
                dir.join(path)
            } else {
                path
            };
            repo.add_path(&path)?;
            let _ = repo.save(&dir)?;
            println!("Added {:?} to staging area", path);
        }

        Commands::Commit { message } => {
            let mut repo = Repository::load(&dir)?;
            let hash = repo.commit(&message)?;
            println!("Created commit {}", &hash[..8]);
        }

        Commands::Status => {
            let repo = Repository::load(&dir)?;
            print!("{}", repo.status());
        }
    }
    Ok(())
}
