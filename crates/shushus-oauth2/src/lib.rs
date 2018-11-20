#![feature(specialization)]

#[macro_use] extern crate failure;
// #[macro_use] extern crate serde_derive;
// extern crate serde;
// extern crate serde_json;
extern crate shushus;
extern crate uuid;
extern crate futures;
// extern crate hyper;
// extern crate hyper_tls;
//
pub mod config;
pub mod error;
pub mod oauth;
// pub mod http;
//
mod states;
//
// pub use config::Config;
pub use error::Error;
// pub use oauth::OAuth;
//
