use actix_web::{web, get, HttpServer, App, HttpResponse};
use cached::Cached;
use cached::stores::SizedCache;
use log::debug;

use shakesperean_pokemon::services;
use shakesperean_pokemon::services::shakesperean_pokemon::ShakesMon;
use shakesperean_pokemon::error::Error;
use std::sync::Mutex;

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
async fn cached_pokemon(info: web::Path::<String>, state: web::Data<AppState>) 
    -> Result<HttpResponse, Error> 
{
    let pokemon = info.into_inner();

    {
        let mut cache = state.cache.lock().unwrap();
        if let Some(result) = cache.cache_get(&pokemon) {
            debug!("Found result for {} in cache", pokemon);
            return Ok(HttpResponse::Ok().json(result));
        }
    }

    let result = services::shakesperean_pokemon::pokemon(&pokemon).await?;

    let mut cache = state.cache.lock().unwrap();
    cache.cache_set(pokemon, result.clone());

    Ok(HttpResponse::Ok().json(result))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::new());

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(cached_pokemon)
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
            App::new().service(cached_pokemon).app_data(web::Data::new(AppState::new()))
        ).await;

        let req = TestRequest::get().uri("/pokemon/charizard").to_request();
        let result: ShakesMon = test::read_response_json(&mut app, req).await;

        assert_eq!(result.name, "charizard");
        assert!(result.description.len() > 0);
    }

    #[actix_rt::test]
    async fn pokemon_endpoint_404() {
        let mut app = test::init_service(
            App::new().service(cached_pokemon).app_data(web::Data::new(AppState::new()))
        ).await;

        let req = TestRequest::get().uri("/pokemon/charizarda").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

