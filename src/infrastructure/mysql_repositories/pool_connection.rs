use actix_web::guard::Guard;
use sqlx::{MySql, Pool};
use std::{
    env,
    sync::{Arc, RwLock},
};

async fn get_pool() -> sqlx::MySqlPool {
    let db_host = env::var("DB_HOST").unwrap_or("localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or("3306".to_string());
    let db_user = env::var("DB_USER").unwrap_or("root".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or("password".to_string());
    let db_name = env::var("DB_NAME").unwrap_or("my_database".to_string());

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );
    sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .expect("Failed to create MySQL connection pool")
}

pub struct GuardPool {
    pool: RwLock<Option<Arc<sqlx::MySqlPool>>>,
}

impl GuardPool {
    pub const fn new() -> Self {
        Self {
            pool: RwLock::new(None),
        }
    }

    pub async fn init(&self) {
        let mut guard = self.pool.write().unwrap();

        if guard.is_none() {
            let pool = get_pool().await;
            *guard = Some(Arc::new(pool));
        }
    }

    pub fn get(&self) -> Arc<sqlx::MySqlPool> {
        self.pool
            .read()
            .unwrap()
            .as_ref()
            .expect("Pool not initialized")
            .clone()
    }
}
