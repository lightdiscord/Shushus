use shushus::http::Uri;
use failure::Fallible;

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "invalid configuration")]
    InvalidConfiguration
}

pub struct Config {
    client_id: String,
    client_secret: String,
    redirect_uri: Uri,
    authorization_uri: Uri,
    token_uri: Uri,
    scopes: Vec<String>
}

#[derive(Default)]
pub struct ConfigBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect_uri: Option<Uri>,
    authorization_uri: Option<Uri>,
    token_uri: Option<Uri>,
    scopes: Vec<String>
}

impl ConfigBuilder {
    pub fn client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }

    pub fn redirect_uri(mut self, redirect_uri: Uri) -> Self {
        self.redirect_uri = Some(redirect_uri);
        self
    }

    pub fn authorization_uri(mut self, authorization_uri: Uri) -> Self {
        self.authorization_uri = Some(authorization_uri);
        self
    }

    pub fn token_uri(mut self, token_uri: Uri) -> Self {
        self.token_uri = Some(token_uri);
        self
    }

    pub fn scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }

    pub fn build(self) -> Fallible<Config> {
        let ConfigBuilder {
            client_id,
            client_secret,
            redirect_uri,
            authorization_uri,
            token_uri,
            scopes
        } = self;

        let check = (client_id, client_secret, redirect_uri, authorization_uri, token_uri);

        if let (
            Some(client_id),
            Some(client_secret),
            Some(redirect_uri),
            Some(authorization_uri),
            Some(token_uri)
        ) = check {
            let config = Config {
                client_id,
                client_secret,
                redirect_uri,
                authorization_uri,
                token_uri,
                scopes
            };

            Ok(config)
        } else {
            Err(ConfigError::InvalidConfiguration)?
        }
    }
}
