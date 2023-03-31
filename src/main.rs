use std::env;

use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};
use log::info;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SimpleMsgReturn {
    msg: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    let response = SimpleMsgReturn {
        msg: "I'm alive!".to_string(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let host: String = env::var("RUSTYAUTH_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("RUSTYAUTH_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| panic!("Failed to parse server port number"));

    info!(
        "Starting Rusty Authenticator Microservice on: {}:{}",
        host, port
    );

    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((host, port))?
    .run()
    .await
}
