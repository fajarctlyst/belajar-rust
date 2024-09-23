use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Temperature {
    fahrenheit: f32,
    celsius: f32,
}

#[get("/to-celsius/{fahrenheit}")]
async fn to_celsius(f: web::Path<f32>) -> impl Responder {
    let f = f.into_inner();
    let c = (f - 32.0) / 1.8;
    web::Json(Temperature {
        celsius: c,
        fahrenheit: f,
    })
}

#[get("/to-fahrenheit/{celsius}")]
async fn to_fahrenheit(c: web::Path<f32>) -> impl Responder {
    let c = c.into_inner();
    let f = 32.0 + (c * 1.8);
    web::Json(Temperature {
        celsius: c,
        fahrenheit: f,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(to_celsius).service(to_fahrenheit))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
