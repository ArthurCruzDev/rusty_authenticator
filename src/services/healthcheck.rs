use actix_web::{get, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SimpleMsgReturn {
    msg: String,
}

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    let response = SimpleMsgReturn {
        msg: "I'm alive!".to_string(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
}
