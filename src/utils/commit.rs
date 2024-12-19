use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub timestamp: u64,
    pub parent: Option<String>,
    pub files: HashMap<String, String>,
}
