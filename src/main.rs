use actix_web::{web, get, HttpServer, App, HttpResponse};
use cached::Cached;
use cached::stores::SizedCache;
use log::debug;

use shakesperean_pokemon::services::shakesperean_pokemon::{ShakesMonService, ShakesMon};
use shakesperean_pokemon::error::Error;
use std::sync::Mutex;

const FUN_TRANSLATION_API_KEY: &str = "FUN_TRANSLATION_API_KEY";

struct AppState {
    cache: Mutex<SizedCache<String, ShakesMon>>,
    pokemon_service: ShakesMonService,
}

impl AppState {
    fn new(api_key: Option<String>) -> Self {
        AppState {
            cache: Mutex::new(SizedCache::with_size(2048)),
            pokemon_service: ShakesMonService::new(api_key),
        }
    }
}

#[get("/healthcheck")]
async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
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

    let result = state.pokemon_service.fetch_description(&pokemon).await?;

    let mut cache = state.cache.lock().unwrap();
    cache.cache_set(pokemon, result.clone());

    Ok(HttpResponse::Ok().json(result))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let api_key = std::env::var(FUN_TRANSLATION_API_KEY).ok();
    let app_state = web::Data::new(AppState::new(api_key));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(cached_pokemon)
            .service(healthcheck)
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
            App::new()
            .service(cached_pokemon)
            .app_data(web::Data::new(AppState::new(None)))
        ).await;

        let req = TestRequest::get().uri("/pokemon/charizard").to_request();
        let result: ShakesMon = test::read_response_json(&mut app, req).await;

        assert_eq!(result.name, "charizard");
        assert!(result.description.len() > 0);
    }

    #[actix_rt::test]
    async fn pokemon_endpoint_404() {
        let mut app = test::init_service(
            App::new()
            .service(cached_pokemon)
            .app_data(web::Data::new(AppState::new(None)))
        ).await;

        let req = TestRequest::get().uri("/pokemon/charizarda").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn healthcheck_endpoint() {
        let mut app = test::init_service(
            App::new().service(healthcheck)
        ).await;

        let req = TestRequest::get().uri("/healthcheck").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}

