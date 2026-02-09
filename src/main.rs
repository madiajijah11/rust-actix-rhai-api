use actix_web::{
    HttpServer, get, App, web::Path, Responder
}

use rhai::Engine;

#[get("/multiply/{a}/{b}")]
async fn multiply(path: Path((a, b)): Path<(i64, i64)>) -> impl Responder {
    let engine = Engine::new();
    let result = engine.eval::<i64>(&format!("{} * {}", a, b)).unwrap();
    format!("{}", result)
}

#[get("/add/{a}/{b}")]
async fn add(path: Path((a, b)): Path<(i64, i64)>) -> impl Responder {
    let engine = Engine::new();
    let result = engine.eval::<i64>(&format!("{} + {}", a, b)).unwrap();
    format!("{}", result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(multiply).service(add)
    })
    .bind(("127.0.0.1", 8080)).unwrap().run().await
}