extern crate shushus_oauth2;

use shushus_oauth2::oauth::{ Basic, OAuth };
use shushus_oauth2::Config;

pub struct GitHub<C>(pub C);

impl GitHub<Basic> {
    pub fn new(config: Config) -> Self {
        GitHub(Basic::new(config))
    }
}
