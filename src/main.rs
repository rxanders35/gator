use gator::config::{self, Config};
use sqlx::{Connection, PgConnection};
use std::{collections::HashMap, env, error::Error};

pub struct State {
    pub config: Config,
}

pub struct Command {
    name: String,
    args: Vec<String>,
}

pub struct Commands {}

impl Commands {}

pub fn handle_login(mut state: State, command: Command, path: &str) -> Result<(), String> {
    if command.args.is_empty() {
        return Err("No username provided".to_string());
    }
    let new_user_name = &command.args[1];
    state.config.current_user_name = new_user_name.clone();
    println!("Logged in as {}", new_user_name);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let url = "postgres://reidx:0794@localhost:5432/gatordb";
    let conn = PgConnection::connect(url);

    Ok(())
}
