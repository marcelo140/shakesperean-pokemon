use serde::Serialize;
use actix_web::{web, get, HttpServer, App, HttpResponse};
use shakesperean_pokemon::services;

#[derive(Debug,Serialize)]
struct ShakesMon {
    name: String,
    description: String,
}

impl ShakesMon {
    fn new(name: String, description: String) -> Self {
        ShakesMon {
            name,
            description,
        }
    }
}

#[get("/pokemon/{name}")]
async fn pokemon(info: web::Path::<String>) -> HttpResponse {
    let pokemon = services::pokemon::species(&info).await;
    let flavor = pokemon.flavor_text("en");
    let shaks = services::shakespeare::translate(flavor).await;

    HttpResponse::Ok().json(ShakesMon::new(info.into_inner(), shaks))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(pokemon))
        .bind("localhost:8080")?
        .run()
        .await
}
