use gator::config::Config;
use sqlx::{Connection, PgConnection};
use std::{env, error::Error};

pub struct State {
    pub config: Config,
}

pub struct Command {
    name: String,
    args: Vec<String>,
}

pub fn handle_login(state: State, command: Command) -> Result<(), Box<dyn Error>> {
    if command.args.is_empty() {
        Err("Missing username")
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let url = "postgres://reidx:0794@localhost:5432/gatordb";
    let conn = PgConnection::connect(url);

    Ok(())
}
