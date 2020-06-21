use serde::{Serialize,Deserialize};
use actix_web::{web, get, HttpServer, App, HttpResponse};
use cached::Cached;
use cached::stores::SizedCache;
use log::debug;

use shakesperean_pokemon::services;
use shakesperean_pokemon::error::Error;
use std::sync::Mutex;

#[derive(Debug,Clone,Serialize,Deserialize)]
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

struct AppState {
    cache: Mutex<SizedCache<String, ShakesMon>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            cache: Mutex::new(SizedCache::with_size(2048)),
        }
    }
}

#[get("/pokemon/{name}")]
async fn pokemon(info: web::Path::<String>, state: web::Data<AppState>) 
    -> Result<HttpResponse, Error> 
{
    let mut cache = state.cache.lock().unwrap();
    let pokemon_name = info.into_inner();
    
    if let Some(result) = cache.cache_get(&pokemon_name) {
        let response = HttpResponse::Ok().json(result);

        debug!("Found result for {} in cache", pokemon_name);
        return Ok(response);
    }

    let pokemon = services::pokemon::species(&pokemon_name).await?;
    match pokemon.flavor_text("en") {
        Some(text) => {
            let translation = services::shakespeare::translate(text).await?;

            let result = ShakesMon::new(pokemon_name.clone(), translation);
            let response = HttpResponse::Ok().json(result.clone());

            cache.cache_set(pokemon_name, result);
            Ok(response)
        },
        None => Err(Error::no_flavor(&pokemon_name)),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::new());

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(pokemon)
    })
    .bind("localhost:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, test::TestRequest, App, http::StatusCode};

    #[actix_rt::test]
    async fn pokemon_endpoint() {
        let mut app = test::init_service(
            App::new().service(pokemon).app_data(web::Data::new(AppState::new()))
        ).await;

        let req = TestRequest::get().uri("/pokemon/charizard").to_request();
        let result: ShakesMon = test::read_response_json(&mut app, req).await;

        assert_eq!(result.name, "charizard");
        assert!(result.description.len() > 0);
    }

    #[actix_rt::test]
    async fn pokemon_endpoint_404() {
        let mut app = test::init_service(
            App::new().service(pokemon).app_data(web::Data::new(AppState::new()))
        ).await;

        let req = TestRequest::get().uri("/pokemon/charizarda").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
