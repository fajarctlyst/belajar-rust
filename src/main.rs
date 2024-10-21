use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
struct Temperature {
    fahrenheit: f32,
    celsius: f32,
}

#[derive(Default)]
struct UsageStats {
    counters: Mutex<Counters>,
}

impl UsageStats {
    fn new() -> Self {
        UsageStats::default()
    }
}

#[derive(Default)]
struct Counters {
    to_fahrenheit: u32,
    to_celsius: u32,
}

#[derive(Serialize)]
struct UsageStatsResponse {
    to_fahrenheit: u32,
    to_celsius: u32,
}

#[get("/to-celsius/{fahrenheit}")]
async fn to_celsius(f: web::Path<f32>, stats: web::Data<UsageStats>) -> impl Responder {
    actix_web::rt::spawn(async move {
        let mut counts = stats.counters.lock().unwrap();
        counts.to_celsius += 1;
    });

    let f = f.into_inner();
    let c = (f - 32.0) / 1.8;
    web::Json(Temperature {
        celsius: c,
        fahrenheit: f,
    })
}

#[get("/to-fahrenheit/{celsius}")]
async fn to_fahrenheit(c: web::Path<f32>, stats: web::Data<UsageStats>) -> impl Responder {
    actix_web::rt::spawn(async move {
        let mut counts = stats.counters.lock().unwrap();
        counts.to_fahrenheit += 1;
    });

    let c = c.into_inner();
    let f = 32.0 + (c * 1.8);
    web::Json(Temperature {
        celsius: c,
        fahrenheit: f,
    })
}

#[get("/usage-statistics")]
async fn usage_statistics(stats: web::Data<UsageStats>) -> impl Responder {
    let mut counts = stats.counters.lock().unwrap();

    let response = UsageStatsResponse {
        to_fahrenheit: counts.to_fahrenheit,
        to_celsius: counts.to_celsius,
    };

    counts.to_fahrenheit = 0;
    counts.to_celsius = 0;

    web::Json(response)
}

#[post("/reset-usage-statistics")]
async fn reset_usage_statistics(stats: web::Data<UsageStats>) -> impl Responder {
    let mut counts = stats.counters.lock().unwrap();

    counts.to_fahrenheit = 0;
    counts.to_celsius = 0;

    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counts = web::Data::new(UsageStats::new());

    HttpServer::new(move || {
        App::new()
            .app_data(counts.clone())
            .service(to_celsius)
            .service(to_fahrenheit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
