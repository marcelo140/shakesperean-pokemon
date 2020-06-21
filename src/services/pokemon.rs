use serde::Deserialize;
use log::debug;

use crate::error::Error;

const SPECIES_ENDPOINT: &str = "https://pokeapi.co/api/v2/pokemon-species/";

/// A (partial) representation of a Pokémon from PokéAPI.
#[derive(Debug,Clone,Deserialize)]
pub struct Pokemon {
    flavor_text_entries: Vec<FlavorText>,
}

#[derive(Debug,Clone,Deserialize)]
struct FlavorText {
    flavor_text: String,
    language: Language,
}

#[derive(Debug,Clone,Deserialize)]
struct Language {
    name: String,
}

impl Pokemon {
    /// Returns the flavor text from the latest generation of Pokémon, as these are
    /// usually more complete.
    pub fn flavor_text(&self, lang: &str) -> Option<&str> {
        self.flavor_text_entries.iter()
            .rev()
            .find(|ft| ft.language.name == lang)
            .map(|ft| ft.flavor_text.as_str())
    }
}

/// Fetches information about a Pokémon from PokéAPI. 
pub async fn species(pokemon: &str) -> Result<Pokemon, Error> {
    debug!("Looking for {} in PokeAPI", pokemon);

    let url = SPECIES_ENDPOINT.to_owned() + pokemon;

    let response = reqwest::get(&url)
        .await?;

    if response.status().as_u16() == 404 {
        return Err(Error::no_pokemon(pokemon));
    }

    let pokemon = response.json::<Pokemon>().await?;
    Ok(pokemon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flavor_text() {
        let pokemon = Pokemon {
            flavor_text_entries: vec![
                FlavorText {
                    flavor_text: "placeholder_1".to_owned(),
                    language: Language {
                        name: "en".to_owned(),
                    },
                },
                FlavorText {
                    flavor_text: "placeholder_2".to_owned(),
                    language: Language {
                        name: "en".to_owned(),
                    },
                },
                FlavorText {
                    flavor_text: "placeholder_3".to_owned(),
                    language: Language {
                        name: "fr".to_owned(),
                    },
                },
            ],
        };

        assert_eq!(pokemon.flavor_text("en"), Some("placeholder_2"));
    }

    #[actix_rt::test]
    async fn external_api_test() {
        let charizard = species("charizard").await;
        assert!(charizard.is_ok());
    }
}
