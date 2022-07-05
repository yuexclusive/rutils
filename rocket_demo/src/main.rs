#[macro_use]
extern crate rocket;
pub mod common;
pub mod config;
pub mod controller;
pub mod dao;
pub mod service;

use controller::common::{self as common_controller, JsonResult};
use controller::user;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[get("/ping")]
pub async fn ping() -> JsonResult<()> {
    common_controller::ok_with_msg("pong!!")
}

// #[catch(401)]
// fn unauthorized(req: &rocket::Request) -> String {
//     let res = block_on(req.guard());
//     format!("Unauthorized'{}'. ", req.uri())
// }

pub fn rocket_cors_config() -> rocket_cors::Cors {
    //这里建议自己去看文档,Rust的文档写的都蛮不错的
    rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("激励太没,O~,baby~")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let _ = rocket::build()
        .register("/", catchers![])
        .mount("/", routes![ping, user::login, user::check])
        .mount(
            "/user",
            routes![user::query, user::get, user::insert, user::update_pwd],
        )
        .attach(rocket_cors_config())
        .launch()
        .await?;
    Ok(())
}
