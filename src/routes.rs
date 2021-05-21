use crate::lib::AppState;
use actix_web::{error, get, post, web, Error, HttpResponse};
use futures::StreamExt;
const MAX_SIZE: usize = 10_000_000;

#[post("/{key}")]
pub async fn write(
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

    match String::from_utf8(body.to_vec()) {
        Ok(value) => {
            let key_copy = key.clone();
            data.db.set(key, value, None).await;
            println!("{:?}", data.db.get(&key_copy).await);
            Ok(HttpResponse::Ok().into())
        }
        Err(_) => Err(HttpResponse::BadRequest().into()),
    }
}

#[get("/{key}")]
pub async fn read(
    web::Path(key): web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    match data.db.get(&key).await {
        Some(val) => Ok(HttpResponse::Ok().body(val)),
        None => Err(HttpResponse::NotFound().into()),
    }
}
