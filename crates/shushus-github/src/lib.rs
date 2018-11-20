extern crate shushus;
extern crate shushus_oauth2;
extern crate failure;

use shushus::http::Uri;
use failure::Fallible;
//
// use std::str::FromStr;
// use shushus_oauth2::oauth::{ Basic, OAuth };
// use shushus_oauth2::config::ConfigBuilder;
// use shushus_oauth2::oauth::{ Authentify, FetchProfile };
//
// pub struct GitHub<C>(pub C);
//
// impl GitHub<Basic> {
//     pub fn new(config: ConfigBuilder) -> Fallible<Self> {
//         let authorization_url = "https://github.com/login/oauth/authorize";
//         let token_url = "https://github.com/login/oauth/access_token";
//
//         let config = config
//             .authorization_url(Uri::from_str(authorization_url)?)
//             .token_url(Uri::from_str(token_url)?)
//             .finalize()?;
//
//         Ok(GitHub(Basic::new(config)))
//     }
// }
//
// impl<C: Authentify> Authentify for GitHub<C> {
//
// }
//

use shushus_oauth2::config::{ Config, ConfigBuilder };

pub struct GitHub {
    client_secret: String,
    client_id: String,
    redirect_uri: Uri,
    scopes: Vec<String>
}

impl GitHub {
    fn new(client_secret: String, client_id: String, redirect_uri: Uri, scopes: Vec<String>) -> Fallible<Self> {
        let github = GitHub {
            client_secret,
            client_id,
            redirect_uri,
            scopes,
        };

        github.config()
            .map(|_| github)
    }

    fn config(&self) -> Fallible<Config> {
        let authorization_uri = "https://github.com/login/oauth/authorize".parse()?;
        let token_uri = "https://github.com/login/oauth/access_token".parse()?;

        ConfigBuilder::default()
            .authorization_uri(authorization_uri)
            .token_uri(token_uri)
            .client_secret(self.client_secret.clone())
            .client_id(self.client_id.clone())
            .redirect_uri(self.redirect_uri.clone())
            .scopes(self.scopes.clone())
            .build()
    }
}

