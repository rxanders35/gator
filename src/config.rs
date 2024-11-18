use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{fs, io::BufReader, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub current_user_name: String,
}

pub fn read(path: &str) -> Result<Config> {}

impl Config {
    pub fn set_user(current_user_name: &str, path: &str) -> Result<Config> {
        let save = serde_json::to_string_pretty(user).unwrap();
    }
}
