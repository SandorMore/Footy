#[macro_use] extern crate rocket;

pub mod credentials_manager;
pub mod database_manager;
pub mod guard;
pub mod routes;

use sqlx::postgres::PgPoolOptions;

pub struct AppConfig {
    pub jwt_secret: String,
    pub pool: sqlx::PgPool,
}

#[rocket::main]
async fn main() {
    dotenvy::dotenv().ok();


    let pool = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("failed to connect to db");

    println!("Connected to NeonDB!");


    let config = AppConfig {
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET not set"),
        pool,
    };

    let _ = rocket::build()
        .manage(config)
        .mount("/", routes![
            routes::register,
            routes::login,
            routes::retrieve_players,
        ])
        .launch()
        .await;
}