use serde::{Serialize,Deserialize};
use reqwest::{Url, Client};

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

const TRANSLATE_ENDPOINT: &str = 
    "https://api.funtranslations.com/translate/shakespeare.json";

pub async fn translate(text: &str) -> String {
    let url = Url::parse(TRANSLATE_ENDPOINT).unwrap();

    let client = Client::new();
    client.post(url)
        .form(&TranslationPayload::new(text.to_owned()))
        .send()
        .await
        .unwrap()
        .json::<TranslationResponse>()
        .await
        .unwrap()
        .contents
        .translated
}
