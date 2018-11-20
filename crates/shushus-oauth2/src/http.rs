use hyper::client::{ Client, HttpConnector };
use hyper::Body;
use hyper_tls::HttpsConnector;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

pub fn client() -> Client<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new(4).unwrap();

    Client::builder()
        .build::<_, Body>(https)
}
