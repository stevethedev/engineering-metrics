use std::time::Duration;

use lib_database::{Connection, Options, Result};
use lib_environment::{DbConnectionString, EnvironmentVariable};

pub async fn open() -> Result<Connection> {
    log::info!("Opening database connection");
    let env_db_connection_string = DbConnectionString::get();
    let mut options = Options::new(env_db_connection_string);
    options.with_max_connections(5);
    options.with_min_connections(1);
    options.with_connect_timeout(Duration::from_secs(5));
    let db_connection = Connection::connect(options).await?;
    log::info!("Database connection established");

    log::info!("Applying migrations");
    db_connection.migrate().await?;
    log::info!("Migrations complete");

    Ok(db_connection)
}
