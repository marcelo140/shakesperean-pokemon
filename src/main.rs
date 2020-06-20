use actix_web::{web, get, HttpServer, App, Responder};

#[get("/pokemon/{name}")]
async fn pokemon(info: web::Path::<String>) -> impl Responder {
    format!("Hello {}", info)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(pokemon))
        .bind("localhost:8080")?
        .run()
        .await
}
