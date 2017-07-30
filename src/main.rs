// main.rs
extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

fn main() {
  Iron::new(|_: &mut Request| {
    let content_type = "application/json".parse::<Mime>().unwrap();

    Ok(Response::with((content_type, status::Ok, "{ \"response\": \"So Cool!\" }")))
  }).http("localhost:3002").unwrap();
}