use crate::utils::commit::Commit;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub current_commit: Option<String>,
    pub commits: HashMap<String, Commit>, // message -> commit data
    pub objects: HashMap<String, Vec<u8>>, // hash -> content
    pub staging: HashMap<String, String>, // filename -> content hash
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            current_commit: None,
            staging: HashMap::new(),
            commits: HashMap::new(),
            objects: HashMap::new(),
        }
    }

    pub fn init(path: &Path) -> std::io::Result<Self> {
        let dir = path.join(".gud");
        fs::create_dir_all(&dir)?;
        let repo = Repository::new();
        repo.save(path)?;
        Ok(repo)
    }

    pub fn commit(&mut self, message: &str) -> std::io::Result<String> {
        if self.staging.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Nothing to commit!",
            ));
        }

        let commit = Commit {
            hash: String::new(),
            message: message.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            parent: self.current_commit.clone(),
            files: self.staging.clone(),
        };

        let commit_hash = self.hash_string(&serde_json::to_string(&commit).unwrap());
        let commit = Commit {
            hash: commit_hash.clone(),
            ..commit
        };

        self.commits.insert(commit_hash.clone(), commit);
        self.current_commit = Some(commit_hash.clone());
        self.staging.clear();
        self.save(&std::env::current_dir()?)?;

        Ok(commit_hash)
    }

    pub fn add_path(&mut self, path: &Path) -> std::io::Result<()> {
        if path.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && !path.starts_with(".gud") {
                    self.add_file(&path)?;
                }
            }
        } else {
            self.add_file(path)?;
        }
        Ok(())
    }

    pub fn add_file(&mut self, path: &Path) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        let hash = self.hash_object(&content);
        self.objects.insert(hash.clone(), content);
        self.staging.insert(
            path.file_name().unwrap().to_string_lossy().to_string(),
            hash,
        );
        Ok(())
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let dir = path.join(".gud");
        let repo_file = dir.join("repository.json");
        let serialised = serde_json::to_string(self)?;
        let _ = fs::write(repo_file, serialised);
        Ok(())
    }

    pub fn load(path: &Path) -> std::io::Result<Self> {
        let dir = path.join(".gud");
        let repo_file = dir.join("repository.json");
        let content = fs::read_to_string(repo_file)?;
        let repo: Repository = serde_json::from_str(&content)?;
        Ok(repo)
    }

    pub fn status(&self) -> String {
        let mut status = String::new();
        status.push_str("Changes staged for commit:\n");
        for file in self.staging.keys() {
            status.push_str(&format!(" {}\n", file));
        }
        status
    }

    fn hash_object(&self, content: &[u8]) -> String {
        let mut hasher = Sha1::new();
        hasher.input(content);
        hasher.result_str()
    }

    fn hash_string(&self, content: &str) -> String {
        self.hash_object(content.as_bytes())
    }
}
