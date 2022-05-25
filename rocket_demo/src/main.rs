#![allow(dead_code)]
#![allow(unused)]
#[macro_use]
extern crate rocket;
pub mod config;
pub mod controller;
pub mod dao;
pub mod service;

use rocket::form::Form;
use rocket::http::Header;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Config;
use rocket::Request;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[get("/hello_path/<name>")]
fn hello_path(name: &str) -> String {
    format!("hello {}", name)
}
#[get("/hello_query_string?<name>")]
fn hello_query_string(name: &str) -> String {
    format!("hello {}", name)
}
#[get("/paths/<paths..>")]
fn paths(paths: PathBuf) -> String {
    let v: Vec<&str> = paths.iter().map(|name| name.to_str().unwrap()).collect();
    let res = v.join(",");
    res
}
#[derive(FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    complete: bool,
    r#type: &'r str,
}
#[post("/form", data = "<task>")]
fn form(task: Form<Task<'_>>) -> String {
    format!("type: {}, complete: {}", task.r#type, task.complete)
}

#[post("/json_req", data = "<task>")]
fn json_req(task: Json<Task<'_>>) -> String {
    format!("type: {}, complete: {}", task.r#type, task.complete)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[get("/json_res")]
fn json_res() -> Json<Person<'static>> {
    Json(Person {
        name: "test",
        age: 18,
    })
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[catch(404)]
fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#,
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                hello_path,
                hello_query_string,
                paths,
                form,
                json_req,
                json_res,
                controller::user::query,
            ],
        )
        .register("/", catchers![general_not_found, default_catcher])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_path() {
        /* .. */
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/hello_path/world").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "hello world");
    }

    #[test]
    fn hello_query_string() {
        /* .. */
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/hello_query_string?name=world").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "hello world");
    }

    #[test]
    fn paths() {
        /* .. */
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/paths/n1/n2").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "n1,n2");
    }

    #[test]
    fn json_req() {
        /* .. */
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut req = client.post("/json_req");
        req.add_header(Header::new("Content-Type", "application/json"));

        req.set_body(r#"{"type":"message","complete":false}"#);
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            "type: message, complete: false"
        );
    }

    #[test]
    fn json_res() {
        /* .. */
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut req = client.get("/json_res");

        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            r#"{"name":"test","age":18}"#
        );
    }
}
