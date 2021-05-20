use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Mutex;

struct AppState {
    db: Mutex<HashMap<String, String>>,
}

const MAX_SIZE: usize = 10_000_000;

#[post("/{key}")]
async fn write(
    web::Path(key): web::Path<String>,
    mut payload: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let mut db = data.db.lock().unwrap();
    match String::from_utf8(body.to_vec()) {
        Ok(value) => {
            db.insert(key, value);
            Ok(HttpResponse::Ok().into())
        }
        Err(_) => Err(HttpResponse::BadRequest().into()),
    }
}

#[get("/{key}")]
async fn read(
    web::Path(key): web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let db = data.db.lock().unwrap();
    match db.get(&key) {
        Some(val) => Ok(HttpResponse::Ok().body(val)),
        None => Err(HttpResponse::NotFound().into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let prefilled_db = HashMap::new();
    let db = web::Data::new(AppState {
        db: Mutex::new(prefilled_db),
    });

    HttpServer::new(move || App::new().app_data(db.clone()).service(write).service(read))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
