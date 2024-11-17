use serde::{Deserialize, Serialize};
use std::{fs, io::BufReader, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = db_url)]
    pub db_url: String,
    #[serde(rename = current_user_name)]
    pub current_user_name: String,
}

pub fn read(path: &str) -> Result<Config> {
    let file = File::open(path);
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader);

    Ok(config)
}

impl Config {
    pub fn set_user(&mut self, user: &str, path: &str) -> Result<()> {
        self.current_user_name = user.to_string();
        let json = serde_json::to_string_pretty(self);
    }
}
