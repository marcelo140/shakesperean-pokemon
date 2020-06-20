use serde::Serialize;
use actix_web::{web, get, HttpServer, App, HttpResponse};

use shakesperean_pokemon::services;
use shakesperean_pokemon::error::Error;

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
async fn pokemon(info: web::Path::<String>) -> Result<HttpResponse, Error> {
    let pokemon = services::pokemon::species(&info).await?;
    let flavor = pokemon.flavor_text("en");

    match flavor {
        Some(ft) => {
            let translation = services::shakespeare::translate(ft).await?;
            let response = HttpResponse::Ok()
                .json(ShakesMon::new(info.into_inner(), translation));

            Ok(response)
        },
        None => Err(Error::no_flavor(&info)),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().service(pokemon))
        .bind("localhost:8080")?
        .run()
        .await
}
