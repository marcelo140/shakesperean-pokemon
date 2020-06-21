use serde::{Serialize,Deserialize};
use reqwest::{Url, Client, StatusCode};
use log::debug;

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

#[derive(Debug,Deserialize)]
struct TranslationError {
    error: TranslationErrorDescription,
}

#[derive(Debug,Deserialize)]
struct TranslationErrorDescription {
    code: u16,
    message: String,
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
    debug!("Performing shakespeare translation through FunTranslations API");

    let url = Url::parse(TRANSLATE_ENDPOINT).unwrap();
    let client = Client::new();

    let response = client.post(url)
        .form(&TranslationPayload::new(text.to_owned()))
        .send()
        .await?;

    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        let mut msg: TranslationError = response.json().await?;
        return Err(Error::translation_api_rate(&msg.error.message.split_off(19)));
    }

    let translation: TranslationResponse = response.json().await?;
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
