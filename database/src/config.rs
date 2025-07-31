use std::env;
use dotenvy::dotenv;

// NOTE: Configuration module for the database crate.
// This module defines the configuration structure and implements default values for the database connection.

// Structure representing the database configuration
pub struct Config {
    pub database_url: String,
}

// Implement a default configuration for the database
impl Default for Config {
    fn default() -> Self {
        dotenv().ok(); // Load environment variables from a .env file if it exists
        // Attempt to read the DATABASE_URL from the environment variables
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!("DATABASE_URL environment variable must be set");
        });

        // Return a new instance of Config with the database URL
        Self {
            database_url,
        }
    }
}