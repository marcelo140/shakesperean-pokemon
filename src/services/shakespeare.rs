use serde::{Serialize,Deserialize};
use reqwest::{Url, Client};

use crate::error::Error;

const TRANSLATE_ENDPOINT: &str = 
    "https://api.funtranslations.com/translate/shakespeare.json";

#[derive(Debug,Serialize)]
struct TranslationPayload {
    text: String,
}

#[derive(Debug,Deserialize)]
struct TranslationResponse {
    contents: TranslationContents,
}

#[derive(Debug,Deserialize)]
struct TranslationContents {
    translated: String,
}

impl TranslationPayload {
    fn new (text: String) -> Self {
        TranslationPayload {
            text,
        }
    }
}

/// Translates the input into shakespearan using FunTranslations API.
pub async fn translate(text: &str) -> Result<String, Error> {
    let url = Url::parse(TRANSLATE_ENDPOINT).unwrap();
    let client = Client::new();

    let response = client.post(url)
        .form(&TranslationPayload::new(text.to_owned()))
        .send()
        .await?;

    let translation = response
        .json::<TranslationResponse>()
        .await?;

    Ok(translation.contents.translated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn external_api_test() {
        let translation = translate("A room without books is like a body without a soul.")
            .await;

        assert!(translation.is_ok());
    }
}
