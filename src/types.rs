use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub port: String,
    pub ext: Vec<String>,
    pub dir: String,
    pub command: String,
    pub command_path: String,
    pub ignore_folders: Option<Vec<String>>
}
