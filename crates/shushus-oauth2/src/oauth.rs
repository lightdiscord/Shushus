use shushus::prelude::*;
// use shushus::redirection::Parameters;
// use shushus::http::{ header, uri::Uri };
//
use states::States;
// use http::{ Response, client };
use config::Config;
// use error::Error;
//
// use futures::prelude::*;
// use failure::Fallible;
// use uuid::Uuid;
// use std::str::FromStr;
//
// pub struct OAuth<B> {
//     pub backend: B,
// }
//
// impl<B> OAuth<B> {
//     pub fn new(backend: B) -> Self {
//         OAuth {
//             backend,
//         }
//     }
//
// }
//
// pub struct Basic {
//     config: Config,
//     states: States,
// }
//
// impl Basic {
//     pub fn new(config: Config) -> Self {
//         Basic {
//             config,
//             states: States::default()
//         }
//     }
// }
//
//
pub trait FetchProfile<'r> {
    type Profile;

    fn fetch(&self, raw: &'r str) -> Self::Profile;
}
//
// pub trait Authentify<H: Redirect + FetchQueryParameter> {
//     type Reply;
//
//     fn authenticate(&self, handler: H) -> Fallible<<H as Redirect>::Response>;
//
//     fn callback<F>(&self, handler: H) -> Fallible<&Future<Item = Self::Reply, Error = failure::Error>>;
// }
//
// pub trait Authentify {
//
// }

use futures::prelude::Future;
use failure::{ Fallible, Error };

pub struct Reply<'r, P: 'r> {
    pub raw: &'r str,
    pub profile: P
}

pub trait Authentify<'r, Handler: Redirect + FetchQueryParameter> {
    type Profile: 'r;
    type CallbackResult: Future<Item = Reply<'r, Self::Profile>, Error = Error>;

    fn login(&self, handler: Handler) -> Fallible<<Handler as Redirect>::Response>;

    fn callback(&self, handler: Handler) -> Self::CallbackResult;
}

pub struct Basic {
    config: Config,
    states: States,
}

impl<'r, H: Redirect + FetchQueryParameter> Authentify<'r, H> for Basic {
    type Profile = ();
    type CallbackResult = Box<Future<Item = Reply<'r, Self::Profile>, Error = Error>>;

    fn login(&self, handler: H) -> Fallible<H::Response> {
        unimplemented!()
    }

    fn callback(&self, handler: H) -> Self::CallbackResult {
        let reply = Reply {
            raw: ":wave:",
            profile: ()
        };

        Box::new(futures::future::ok(reply))
    }

    // type Reply = Response;

    // fn authenticate(&self, handler: H) -> Fallible<H::Response> {
    //     let state = self.states.create();

    //     let mut parameters = Parameters::default();
    //     parameters.insert("client_id", self.config.client_id.clone());
    //     parameters.insert("redirect_uri", self.config.redirect_url.to_string());
    //     parameters.insert("state", state.to_string());

    //     let uri = format!("{}{}", self.config.authorization_url, parameters.to_string());

    //     Ok(handler.redirect(Uri::from_str(&uri)?))
    // }

    // fn callback<F>(&self, handler: H) -> Fallible<&Future<Item = Self::Reply, Error = failure::Error>> {
    //     let code = handler.fetch("code").ok_or(Error::InvalidCode)?;
    //     let state = handler.fetch("state").ok_or(Error::InvalidState)?;
    //     let uuid = Uuid::from_str(&state)?;

    //     if !self.states.check(&uuid) {
    //         Err(Error::InvalidState)?;
    //     }

    //     let mut parameters = Parameters::default();
    //     parameters.insert("client_id", self.config.client_id.clone());
    //     parameters.insert("client_secret", self.config.client_secret.clone());
    //     parameters.insert("code", code);
    //     parameters.insert("state", state);
    //     self.states.apply(&uuid);

    //     let url = format!("{}{}", self.config.token_url, parameters.to_string());

    //     let request = hyper::Request::post(url)
    //         .header(header::ACCEPT, "application/json")
    //         .body(hyper::Body::empty())?;

    //     let future = client()
    //         .request(request)
    //         .and_then(|res| res.into_body().concat2())
    //         .from_err::<failure::Error>()
    //         .and_then(|body| {
    //             let response: Response = serde_json::from_slice(&body)?;

    //             Ok(response)
    //         })
    //         .from_err();

    //     Ok(&future)
    // }
}
