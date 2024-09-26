use actix_web::dev::ServiceRequest;
use actix_web::web::scope;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::Serialize;

use std::sync::Mutex;

mod api_tokens;

async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let token = credentials.user_id();

    match api_tokens::is_token_allowed_access(token) {
        Ok(true) => Ok(req),
        Ok(false) => Err((
            actix_web::error::ErrorUnauthorized("Supplied token is not authorized."),
            req,
        )),
        Err(_) => Err((actix_web::error::ErrorInternalServerError(""), req)),
    }
}

#[derive(Serialize)]
struct Temperature {
    fahrenheit: f32,
    celsius: f32,
}

struct UsageStats {
    to_fahrenheit: Mutex<u32>,
    to_celsius: Mutex<u32>,
}

#[derive(Serialize)]
struct UsageStatsResponse {
    to_fahrenheit: u32,
    to_celsius: u32,
}

#[get("/to-celsius/{fahrenheit}")]
async fn to_celsius(f: web::Path<f32>, stats: web::Data<UsageStats>) -> impl Responder {
    actix_web::rt::spawn(async move {
        let mut count = stats.to_celsius.lock().unwrap();
        *count += 1;
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
        let mut count = stats.to_fahrenheit.lock().unwrap();
        *count += 1;
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
    let mut fahrenheit_count = stats.to_fahrenheit.lock().unwrap();
    let mut celsius_count = stats.to_fahrenheit.lock().unwrap();

    let response = UsageStatsResponse {
        to_fahrenheit: *fahrenheit_count,
        to_celsius: *celsius_count,
    };

    *fahrenheit_count = 0;
    *celsius_count = 0;

    web::Json(response)
}

#[post("/reset-usage-statistics")]
async fn reset_usage_statistics(stats: web::Data<UsageStats>) -> impl Responder {
    let mut fahrenheit_count = stats.to_fahrenheit.lock().unwrap();
    let mut celsius_count = stats.to_fahrenheit.lock().unwrap();

    *fahrenheit_count = 0;
    *celsius_count = 0;

    HttpResponse::NoContent()
}

#[get("/api-token")]
async fn request_token() -> actix_web::Result<impl Responder> {
    let token = api_tokens::create_token();

    api_tokens::store_token(&token)?;

    Ok(token + "\r\n")
}

#[delete("/api-token")]
async fn delete_token(auth: BasicAuth) -> actix_web::Result<impl Responder> {
    let token = auth.user_id();

    api_tokens::revoke_token(token)?;

    Ok(HttpResponse::NoContent().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counts = web::Data::new(UsageStats {
        to_fahrenheit: Mutex::new(0),
        to_celsius: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counts.clone())
            .service(
                scope("/api")
                    .wrap(HttpAuthentication::basic(validator))
                    .service(to_fahrenheit)
                    .service(to_celsius),
            )
            .service(request_token)
            .service(delete_token)
            .service(usage_statistics)
            .service(reset_usage_statistics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
