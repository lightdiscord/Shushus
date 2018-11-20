// #![feature(proc_macro_hygiene, decl_macro)]
//
// #[macro_use] extern crate rocket;
// extern crate failure;
// extern crate shushus_oauth2;
// extern crate shushus_rocketrs;
// extern crate shushus_github;
// extern crate shushus;
// extern crate rocket_futures as rfutures;
// extern crate futures;
//
// use rocket::{ Rocket, State };
// use rocket::response::Redirect;
// use shushus::http::uri::Uri;
// use shushus_oauth2::{ OAuth, Config };
// use shushus_oauth2::oauth::Basic;
// use shushus_github::GitHub;
// use shushus_rocketrs::Handler;
// use futures::Future;
// use failure::{ Fallible, Error };
//
// use std::env;
// use std::str::FromStr;
//
// fn github() -> Fallible<OAuth<GitHub<Basic>>> {
//     let redirect_url = "http://localhost:8000/callback";
//
//     let client_id = env::var("GITHUB_CLIENT_ID");
//     let client_secret = env::var("GITHUB_CLIENT_SECRET");
//
//     let config = Config::builder()
//         .client_id(client_id?)
//         .client_secret(client_secret?)
//         .redirect_url(Uri::from_str(redirect_url)?);
//
//     Ok(OAuth::new(GitHub::new(config)?))
// }
//
// #[get("/login")]
// fn login(oauth: State<OAuth<GitHub<Basic>>>, handler: Handler) -> Fallible<Redirect> {
//     Ok(oauth.backend.0.authenticate(handler)?)
// }
//
// #[get("/callback")]
// fn callback<'r>(oauth: State<OAuth<GitHub<Basic>>>, handler: Handler<'r>) -> Fallible<rfutures::Current<impl Future<Item = String, Error = Error> + 'r>> {
//     let future = oauth.backend.0.callback(handler)?
//         .and_then(|response| Ok(format!("{:?}", response)));
//
//     Ok(rfutures::Wrapper::new(future).into())
// }
//
// fn rocket() -> Fallible<Rocket> {
//     let rocket = rocket::ignite()
//         .manage(github()?)
//         .mount("/", routes![login, callback]);
//
//     Ok(rocket)
// }
//
// fn main() -> Fallible<()> {
//     let rocket = rocket()?;
//
//     Err(rocket.launch())?
// }
//

fn main() {
    println!("Hello world!");
}
