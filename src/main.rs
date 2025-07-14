mod config;
mod routes;
mod handlers;
mod utils;
mod modules;
mod middlewares;
mod constants;

use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use log::{info, error};

use env_logger::Env;
use handlers::not_found;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let db_pool = config::db::init_pool().await;

    match db_pool {
        Ok(pool) => {
            info!("Successfully connected to the database");

            
            let port: u16 = std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number");
        
            HttpServer::new(move || {
                let cors = config::cors::setup_cors();

                App::new()
                    .wrap(cors)
                    .app_data(web::Data::new(pool.clone()))
                    .wrap(Logger::default())
                    .configure(routes::config_route)
                    .default_service(web::route().to(not_found))
            })
            .bind(("127.0.0.1", port))?
            .run()
            .await

        }
        Err(err) => {
            error!("Error connecting to the database: {err}");
            std::process::exit(1);
        }
    }
}
