use crate::config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type PostgresPool = Pool<Postgres>;

pub async fn get_pool() -> PostgresPool {
    let url = match config::get("BUILD").as_str() {
        "dev" => config::get("DEV_DATABASE_URL"),
        "prod" => config::get("PROD_DATABASE_URL"),
        _ => "".to_string(),
    };

    log::info!("{}", url);
    let pool = match PgPoolOptions::new().max_connections(10).connect(&url).await {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            sqlx::migrate!("./migrations").run(&pool).await.unwrap();
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    pool
}

// mongo db util functions
