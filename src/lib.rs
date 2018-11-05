pub extern crate http;

pub mod redirection;

pub use redirection::Redirect;

use std::borrow::Cow;

pub trait FetchQueryParameter {
    fn fetch<S>(&self, parameter: S) -> Option<String> where S: Into<Cow<'static, str>>;
}

pub mod prelude {
    pub use super::{ Redirect, FetchQueryParameter };
}
