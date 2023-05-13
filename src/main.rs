pub mod services;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use services::healthcheck::healthcheck;
use std::env;

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
