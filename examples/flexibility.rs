use actix_web::{get, guard, web, App, HttpServer, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    "Hello, world!"
}

async fn greet_user(username: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", username.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || {
        App::new().service(greet).service(
            web::scope("/admin").service(
                web::resource("/user/{name}")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(greet_user)),
            ),
        )
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
