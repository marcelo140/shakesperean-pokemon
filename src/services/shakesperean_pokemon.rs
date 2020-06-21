use serde::{Serialize,Deserialize};

use crate::error::Error;
use crate::services::{pokemon,shakespeare};

/// Pokémon description in Shakespeare.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ShakesMon {
    /// The name of the Pokémon.
    pub name: String,
    /// The Pokémon's description in Shaskepeare.
    pub description: String,
}

impl ShakesMon {
    fn new(name: String, description: String) -> Self {
        ShakesMon {
            name,
            description,
        }
    }
}

/// Service used to fetch Pokémon's descriptions in Shakespeare.
pub struct ShakesMonService {
    api_key: Option<String>,
}

impl ShakesMonService {
    pub fn new(api_key: Option<String>) -> Self {
        ShakesMonService {
            api_key,
        }
    }

    /// Fetches the description of a Pokémon in Shakespeare.
    pub async fn fetch_description(&self, name: &str) -> Result<ShakesMon, Error> {
        let pokemon = pokemon::species(name).await?;

        let text = pokemon.flavor_text("en")
            .ok_or_else(|| Error::no_flavor(name))?;

        let translation = shakespeare::translate(text, &self.api_key).await?;
        Ok(ShakesMon::new(name.to_owned(), translation))
    }
}
