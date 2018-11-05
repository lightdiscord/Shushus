use serde::{ Deserializer, Deserialize };
use hyper::client::{ Client, HttpConnector };
use hyper::Body;
use hyper_tls::HttpsConnector;

type Scopes = Vec<String>;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub access_token: String,
    pub token_type: String,

    #[serde(deserialize_with = "split")]
    pub scope: Scopes
}

pub fn client() -> Client<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new(4).unwrap();

    Client::builder()
        .build::<_, Body>(https)
}

fn split<'de, D>(deserializer: D) -> Result<Scopes, D::Error> where D: Deserializer<'de> {
    let string = String::deserialize(deserializer)?;

    let parts = string.split(',')
        .filter(|part| !part.is_empty())
        .map(String::from)
        .collect();

    Ok(parts)
}
