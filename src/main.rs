mod lib;
mod routes;
mod tests;
use crate::lib::init_app_data;
use crate::lib::{get_config, Config};
use crate::routes::{read, write};
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Config {
        timeout,
        host,
        port,
    } = get_config();

    let db = init_app_data(timeout);

    HttpServer::new(move || App::new().app_data(db.clone()).service(write).service(read))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
