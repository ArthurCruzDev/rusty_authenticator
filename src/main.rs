use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};
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
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
