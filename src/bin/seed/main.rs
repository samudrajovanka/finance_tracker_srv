mod transaction_categories;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::{transaction_categories::seed_transaction_categories};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await;


    match db_pool {
        Ok(pool) => {
            println!("Successfully connected to the database");
            println!("Seeding the database...");

            seed_transaction_categories(&pool).await
                .expect("Failed to seed transaction categories");

            println!("Database seeding completed successfully");
        }
        Err(err) => {
            eprintln!("Error connecting to the database: {err}");
            std::process::exit(1);
        }
    }
}