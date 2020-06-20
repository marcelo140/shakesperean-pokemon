use serde::Deserialize;

use crate::error::Error;

const SPECIES_ENDPOINT: &str = "https://pokeapi.co/api/v2/pokemon-species/";

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
    pub fn flavor_text(&self, lang: &str) -> Option<&str> {
        self.flavor_text_entries.iter()
            .rev()
            .find(|ft| ft.language.name == lang)
            .map(|ft| ft.flavor_text.as_str())
    }
}

pub async fn species(pokemon: &str) -> Result<Pokemon, Error> {
    let url = SPECIES_ENDPOINT.to_owned() + pokemon;
    let response = reqwest::get(&url)
        .await?;

    if response.status().as_u16() == 404 {
        return Err(Error::no_pokemon(pokemon));
    }

    let pokemon = response.json::<Pokemon>().await?;
    Ok(pokemon)
}

