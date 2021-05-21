use actix_web::web;
pub use r_cache::cache::Cache;
use std::time::Duration;

pub struct AppState {
    pub db: Cache<String, String>,
}

#[derive(Debug)]
pub struct Config {
    pub timeout: Duration,
    pub host: String,
    pub port: String,
}

pub fn get_config() -> Config {
    let timeout = Duration::from_secs(
        std::env::var("TIMEOUT")
            .unwrap_or("1800".to_string())
            .parse::<u64>()
            .unwrap(),
    );
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let config = Config {
        timeout,
        host,
        port,
    };
    println!("{:?}", config);
    config
}

pub fn init_app_data(dur: Duration) -> web::Data<AppState> {
    web::Data::new(AppState {
        db: Cache::new(Some(dur)),
    })
}
