use sqlx::{Connection, PgConnection, Row};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://reidx:0794@localhost:5432/gatordb";
    let mut conn = PgConnection::connect(url).await?;

    Ok(())
}


