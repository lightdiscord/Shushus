// extern crate shushus;
// extern crate rocket;
//
// use shushus::prelude::*;
// use shushus::http::Uri;
// use rocket::{ response, request, Outcome };
// use rocket::request::{ Request, FromRequest };
//
// use std::borrow::{ Cow, Borrow };
//
// pub struct Handler<'r>(Request<'r>);
//
// impl<'r> Redirect for Handler<'r> {
//     type Response = response::Redirect;
//
//     fn redirect(&self, uri: Uri) -> Self::Response {
//         response::Redirect::to(uri.to_string())
//     }
// }
//
// impl<'a, 'r> FromRequest<'a, 'r> for Handler<'r> {
//     type Error = ();
//
//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//         Outcome::Success(Handler(request.clone()))
//     }
// }
//
// impl<'r> FetchQueryParameter for Handler<'r> {
//     fn fetch<S>(&self, parameter: S) -> Option<String> where S: Into<Cow<'static, str>> {
//         let parameter = parameter.into();
//         let parameter: &str = parameter.borrow();
//
//         self.0.get_query_value(parameter)
//             .and_then(|item| item.ok())
//     }
// }
//
