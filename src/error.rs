use actix_web::{HttpResponse,ResponseError};
use serde::Serialize;

use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    PokemonNotFound(String),
    NoFlavorFound(String),
    NetworkError(reqwest::Error),
    TranslationApiRate(String),
}

#[derive(Serialize,Debug)]
struct HttpError {
    error: String, 
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PokemonNotFound(pokemon) =>
                write!(f, "Pokemon {} not found", pokemon),
            Error::NoFlavorFound(pokemon) =>
                write!(f, "No flavor text was found for {}", pokemon),
            Error::NetworkError(err) => 
                write!(f, "Network error: {}", err),
            Error::TranslationApiRate(msg) =>
                write!(f, "FunTranslations API error: {}", msg),
        }
    }
}

impl Error {
    pub fn no_pokemon(pokemon: &str) -> Self {
        Error::PokemonNotFound(pokemon.to_owned())
    }

    pub fn no_flavor(pokemon: &str) -> Self {
        Error::NoFlavorFound(pokemon.to_owned())
    }

    pub fn translation_api_rate(msg: &str) -> Self {
        Error::TranslationApiRate(msg.to_owned())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::NetworkError(err)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::PokemonNotFound(_) => 
                HttpResponse::NotFound().finish(),
            Error::NoFlavorFound(_) =>
                HttpResponse::NoContent().finish(),
            Error::NetworkError(_) =>
                HttpResponse::InternalServerError().finish(),
            Error::TranslationApiRate(msg) => {
                let msg = format!("FunTranslations API rate limit reached: {}", msg);
                HttpResponse::InternalServerError()
                    .json(HttpError { error: msg })
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::NetworkError(src) => Some(src),
            _ => None,
        }
    }
}
