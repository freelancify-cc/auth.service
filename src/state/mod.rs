use crate::{database, config};

#[derive(Clone)] 
pub struct AppState {
    name: String,
    pool: database::PostgresPool,
}

impl AppState {
    pub async fn new() -> Self {
        let pool = database::get_pool().await;
        Self {
            name: config::get("APPNAME"),
            pool
        }
    } 

    pub fn get_app_name(&self) -> &str {
        &self.name
    }

    pub fn get_db(&self) -> &database::PostgresPool {
        &self.pool
    }
}  
