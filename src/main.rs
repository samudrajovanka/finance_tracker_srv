mod config;
mod routes;
mod handlers;
mod utils;
mod modules;
mod middlewares;
mod constants;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use handlers::not_found;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = config::db::init_pool().await;

    match db_pool {
        Ok(pool) => {
            println!("Successfully connected to the database");

            let port: u16 = std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number");

            println!("Server running on port {port}");
            
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(pool.clone()))
                    .configure(routes::config_route)
                    .default_service(web::route().to(not_found))
            })
            .bind(("127.0.0.1", port))?
            .run()
            .await

        }
        Err(err) => {
            eprintln!("Error connecting to the database: {err}");
            std::process::exit(1);
        }
    }
}
