pub mod entities;

use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection() -> anyhow::Result<DatabaseConnection> {
    dotenvy::dotenv()?;

    let database_url = env::var("DATABASE_URL")?;

    let db = Database::connect(database_url).await?;

    Ok(db)
}
