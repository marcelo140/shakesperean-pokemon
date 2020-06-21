use serde::{Serialize,Deserialize};
use actix_web::{web, get, HttpServer, App, HttpResponse};

use shakesperean_pokemon::services;
use shakesperean_pokemon::error::Error;

#[derive(Debug,Serialize,Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, http::StatusCode};

    #[actix_rt::test]
    async fn test_pokemon_endpoint() {
        let mut app = test::init_service(App::new().service(pokemon)).await;

        let req = test::TestRequest::get().uri("/pokemon/charizard").to_request();
        let result: ShakesMon = test::read_response_json(&mut app, req).await;

        assert_eq!(result.name, "charizard");
        assert!(result.description.len() > 0);
    }

    #[actix_rt::test]
    async fn test_pokemon_endpoint_404() {
        let mut app = test::init_service(App::new().service(pokemon)).await;

        let req = test::TestRequest::get().uri("/pokemon/charizarda").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
