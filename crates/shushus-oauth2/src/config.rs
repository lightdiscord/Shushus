use crate::Error;

use shushus::http::Uri;
use failure::Fallible;

macro_rules! builder_fields {
    ($struct:ident => [$(($field:ident, $type:ty)),+]) => {
        impl $struct {
            $(
                pub fn $field(mut self, $field: $type) -> Self {
                    self.$field = Some($field);
                    self
                }
            )+
        }
    };
}

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: Uri,
    pub authorization_url: Uri,
    pub token_url: Uri,
    pub scopes: Vec<String>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Debug)]
pub struct ConfigBuilder {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_url: Option<Uri>,
    pub authorization_url: Option<Uri>,
    pub token_url: Option<Uri>,
    pub scopes: Vec<String>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder {
            client_id: None,
            client_secret: None,
            redirect_url: None,
            authorization_url: None,
            token_url: None,
            scopes: Vec::new()
        }
    }
}

builder_fields! {
    ConfigBuilder => [
        (client_id, String),
        (client_secret, String),
        (redirect_url, Uri),
        (authorization_url, Uri),
        (token_url, Uri)
    ]
}

impl ConfigBuilder {
    pub fn scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }

    // TODO: Error type
    pub fn finalize(self) -> Fallible<Config> {
        let ConfigBuilder {
            client_id,
            client_secret,
            redirect_url,
            authorization_url,
            token_url,
            scopes
        } = self;

        let needed = (
            client_id,
            client_secret,
            redirect_url,
            authorization_url,
            token_url
        );

        if let (
            Some(client_id),
            Some(client_secret),
            Some(redirect_url),
            Some(authorization_url),
            Some(token_url)
        ) = needed {
            let config = Config {
                client_id,
                client_secret,
                redirect_url,
                authorization_url,
                token_url,
                scopes
            };

            Ok(config)
        } else {
            Err(Error::InvalidConfiguration)?
        }
    }
}
