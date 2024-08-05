use actix_web::{get, App, HttpServer};

#[get("/healthz")]
async fn liveness() -> &'static str {
    "ok\r\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || App::new().service(liveness);

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
