use actix_web::{web, get, HttpServer, App, Responder};
use shakesperean_pokemon::services;

#[get("/pokemon/{name}")]
async fn pokemon(info: web::Path::<String>) -> impl Responder {
    let pokemon = services::pokemon::species(&info).await;
    pokemon.flavor_text("en").to_owned()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(pokemon))
        .bind("localhost:8080")?
        .run()
        .await
}
