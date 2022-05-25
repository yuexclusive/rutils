use rocket::serde::{json::Json, Serialize};
use std::error::Error;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Code {
    OK(u16),
    ServerError(u16),
}

#[derive(Serialize, Debug)]
#[serde(tag = "status")]
pub enum Response<T>
where
    T: Serialize,
{
    #[serde(rename(serialize = "ok"))]
    OK(OkResult<T>),
    #[serde(rename(serialize = "error"))]
    Error(ErrResult),
}

#[derive(Serialize, Debug)]
pub struct OkResult<T>
where
    T: Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    code: Code,
}

impl<T> OkResult<T> where T: Serialize {}

#[derive(Serialize, Debug)]
pub struct ErrResult {
    msg: String,
    code: Code,
}

impl ErrResult {
    pub fn new(msg: String, code: Code) -> Self {
        Self { msg, code }
    }
}

pub fn ok_with<T>(data: T) -> Json<Response<T>>
where
    T: Serialize,
{
    let res = {
        let code = Code::OK(200);
        OkResult {
            data: Some(data),
            code: Code::OK(200),
        }
    };
    Json(Response::OK(res))
}

pub fn ok<T>() -> Json<Response<T>>
where
    T: Serialize,
{
    let res = OkResult {
        data: None,
        code: Code::OK(200),
    };
    Json(Response::OK(res))
}

pub fn error<T>(err: impl Error, code: Code) -> Json<Response<T>>
where
    T: Serialize,
{
    let res = ErrResult::new(err.to_string(), code);
    Json(Response::Error(res))
}
