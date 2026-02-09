use actix_web::{App, HttpServer, Responder, get, web::Path};

use rhai::Engine;

#[get("/multiply/{a}/{b}")]
async fn multiply(path: Path<(i64, i64)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let mut engine = Engine::new();
    engine.register_fn("a", move || a);
    engine.register_fn("b", move || b);

    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();
    format!("{}", result)
}

#[get("/add/{a}/{b}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let mut engine = Engine::new();
    engine.register_fn("a", move || a);
    engine.register_fn("b", move || b);

    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();
    format!("{}", result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(multiply).service(add))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}
