use shushus::prelude::*;
use shushus::redirection::Parameters;

use states::States;
use http::{ Response, client };
use hyper::http::{ header, uri::Uri };
use config::Config;
use error::Error;

use futures::prelude::*;
use failure::Fallible;
use uuid::Uuid;
use std::str::FromStr;

pub struct OAuth {
    config: Config,
    states: States,
}

impl OAuth {
    pub fn new(config: Config) -> Self {
        OAuth {
            config,
            states: States::default()
        }
    }

    pub fn authenticate<H>(&self, handler: H) -> Fallible<H::Response> where H: Redirect {
        let state = self.states.create();

        let mut parameters = Parameters::default();
        parameters.insert("client_id", self.config.client_id.clone());
        parameters.insert("redirect_uri", self.config.redirect_url.to_string());
        parameters.insert("state", state.to_string());

        let uri = format!("{}{}", self.config.authorization_url, parameters.to_string());

        Ok(handler.redirect(Uri::from_str(&uri)?))
    }

    pub fn callback<H>(&self, handler: H) -> Fallible<impl Future<Item = Response, Error = failure::Error>> where H: FetchQueryParameter {
        let code = handler.fetch("code").ok_or(Error::InvalidCode)?;
        let state = handler.fetch("state").ok_or(Error::InvalidState)?;
        let uuid = Uuid::from_str(&state)?;

        if !self.states.check(&uuid) {
            Err(Error::InvalidState)?;
        }

        let mut parameters = Parameters::default();
        parameters.insert("client_id", self.config.client_id.clone());
        parameters.insert("client_secret", self.config.client_secret.clone());
        parameters.insert("code", code);
        parameters.insert("state", state);
        self.states.apply(&uuid);

        let url = format!("{}{}", self.config.token_url, parameters.to_string());

        let request = hyper::Request::post(url)
            .header(header::ACCEPT, "application/json")
            .body(hyper::Body::empty())?;

        let future = client()
            .request(request)
            .and_then(|res| res.into_body().concat2())
            .from_err::<failure::Error>()
            .and_then(|body| {
                let response: Response = serde_json::from_slice(&body)?;

                Ok(response)
            })
            .from_err();

        Ok(future)
    }
}
