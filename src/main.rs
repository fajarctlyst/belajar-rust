use std::sync::Mutex;

use actix_web::dev::ServiceRequest;
use actix_web::web::scope;
use actix_web::{delete, error, get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use chrono::Utc;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serialize;

use hello_actix::{
    db, delete_api_key, request_api_key, reset_usage_statistics, to_celsius, to_fahrenheit,
    usage_statistics, validator, UsageStats,
};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file(db::DB_FILE);
    let db_pool = db::Pool::new(manager).unwrap();
    db::setup(db_pool.clone());

    let counts = web::Data::new(UsageStats::new());

    HttpServer::new(move || {
        App::new()
            .app_data(counts.clone())
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                scope("/api")
                    .wrap(HttpAuthentication::basic(validator))
                    .service(to_fahrenheit)
                    .service(to_celsius),
            )
            .service(request_api_key)
            .service(delete_api_key)
            .service(usage_statistics)
            .service(reset_usage_statistics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
