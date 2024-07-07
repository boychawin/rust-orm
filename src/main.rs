use actix_web::{web, App, HttpServer};
use db::Config;
// use diesel::r2d2::Pool;
// use diesel::{
//     r2d2::{ConnectionManager, Pool},
//     PgPoolOptions,
// };
use dotenv::dotenv;

mod db;
mod messages;
mod models;
mod schema;
mod services;
mod utils;

use services::fetch_users;
use sqlx::{postgres::PgPoolOptions, Postgres,Pool};
// use utils::AppState;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🧑🏻‍💻 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(fetch_users)
    })
    .bind(("127.0.0.1", 8083))?
    .run()
    .await
}
