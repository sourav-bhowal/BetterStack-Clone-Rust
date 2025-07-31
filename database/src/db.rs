use diesel::{Connection, ConnectionError, PgConnection};
use crate::config;

// NOTE: This file is part of the `database` crate, which provides database-related functionality for the application.
// Includes the database connection modules.

// Structure representing the database connection
pub struct Database {
    pub connection: PgConnection
}

// Implement basic database connection functionality
impl Database {
    // Creates a new database connection using the default configuration.
    pub fn new() -> Result<Self, ConnectionError> {
        // Load the configuration from the config module
        let config = config::Config::default();

        // Extract the database URL from the configuration
        let database_url = &config.database_url;

        // Establish a connection to the PostgreSQL database using the provided URL
        let connection = PgConnection::establish(&database_url)
        .map_err(|e| {
            eprintln!("Error connecting to {}: {}", database_url, e)
        })
        .expect("Failed to establish a database connection");

        // Return a new instance of Database with the established connection
        Ok(Self { connection })
    }
}