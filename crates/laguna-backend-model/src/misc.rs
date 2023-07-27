use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Laguna {
    pub version: String,
    pub authors: Vec<String>,
    pub license: String,
    pub description: String,
    pub repository: String,
}
