#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate rocket;
pub mod config;
pub mod controller;
pub mod dao;
pub mod model;
pub mod service;

use controller::common::{ok, ok_with_data, ok_with_msg, JsonResult};
use controller::example;
use controller::user;

#[get("/test")]
pub async fn test() -> JsonResult<&'static str> {
    ok_with_data("ok")
}

#[get("/ping")]
pub async fn ping() -> JsonResult<()> {
    ok_with_msg("pong!!")
}

#[launch]
fn rocket() -> _ {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    rocket::build().mount(
        "/",
        routes![
            example::hello_path,
            user::query,
            user::update,
            user::insert,
            user::delete,
            ping,
            test,
        ],
    )
}
