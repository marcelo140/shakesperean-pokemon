use serde::{Serialize,Deserialize};

use crate::error::Error;
use crate::services::{pokemon,shakespeare};

#[derive(Debug,Clone,Serialize,Deserialize)]

/// Pokémon description in Shakesperean.
pub struct ShakesMon {
    pub name: String,
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

/// Fetches the description of a Pokémon in Shakesperean.
pub async fn pokemon(name: &str) -> Result<ShakesMon, Error> {
    let pokemon = pokemon::species(name).await?;

    let text = pokemon.flavor_text("en")
        .ok_or(Error::no_flavor(name))?;

    let translation = shakespeare::translate(text).await?;
    Ok(ShakesMon::new(name.to_owned(), translation))
}

