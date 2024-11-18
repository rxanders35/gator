use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub current_user_name: String,
}

pub fn read(path: &str) -> Result<Config, Box<dyn Error>> {
    if !Path::new(path).exists() {
        return Err(format!("{} does not exist", path).into());
    }
    let file = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&file)?;

    Ok(config)
}

impl Config {
    pub fn set_user(&mut self, new_user_name: &str, path: &str) -> Result<(), Box<dyn Error>> {
        self.current_user_name = new_user_name.to_string();
        let save = serde_json::to_string_pretty(&self)?;
        fs::write(path, save)?;
        Ok(())
    }
}
