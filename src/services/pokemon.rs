use serde::Deserialize;

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
    pub fn flavor_text(&self, lang: &str) -> &str {
        self.flavor_text_entries.iter()
            .rev()
            .find(|ft| ft.language.name == lang)
            .unwrap()
            .flavor_text
            .as_str()
    }
}

pub async fn species(pokemon: &str) -> Pokemon {
    reqwest::get(&(SPECIES_ENDPOINT.to_owned() + pokemon)) 
        .await
        .unwrap()
        .json::<Pokemon>()
        .await
        .unwrap()
}

