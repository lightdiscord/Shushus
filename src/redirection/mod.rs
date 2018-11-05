use http::Uri;

use std::collections::HashMap;
use std::borrow::Cow;

pub struct Parameters(HashMap<&'static str, Cow<'static, str>>);

impl Default for Parameters {
    fn default() -> Self {
        Parameters(HashMap::new())
    }
}

impl Parameters {
    pub fn insert<S>(&mut self, key: &'static str, value: S) where S: Into<Cow<'static, str>> {
        self.0.insert(key, value.into());
    }

    pub fn inner(&self) -> &HashMap<&'static str, Cow<'static, str>> {
        &self.0
    }
}

impl ToString for Parameters {
    fn to_string(&self) -> String {
        let parameters = self.inner().iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&");

        format!("?{}", parameters)
    }
}

pub trait Redirect {
    type Response;

    fn redirect(&self, uri: Uri) -> Self::Response;
}
