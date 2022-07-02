#[macro_use]
extern crate rocket;
pub mod common;
pub mod config;
pub mod controller;
pub mod dao;
pub mod service;

use controller::common::{self as common_controller, JsonResult};
use controller::user;

#[get("/ping")]
pub async fn ping() -> JsonResult<()> {
    common_controller::ok_with_msg("pong!!")
}

// #[catch(401)]
// fn unauthorized(req: &rocket::Request) -> String {
//     let res = block_on(req.guard());
//     format!("Unauthorized'{}'. ", req.uri())
// }

#[launch]
fn rocket() -> _ {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    rocket::build()
        .register("/", catchers![])
        .mount("/", routes![ping, user::login, user::check])
        .mount(
            "/user",
            routes![user::query, user::get, user::insert, user::update_pwd],
        )
}
