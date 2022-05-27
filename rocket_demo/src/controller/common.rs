use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Serialize};
use std::error::Error;

#[derive(Serialize)]
// #[serde(untagged)]
// #[serde(rename(serialize = "ok"))]
pub enum Code {
    #[serde(rename(serialize = "20000"))]
    Ok = 20000,
    #[serde(rename(serialize = "50000"))]
    Err = 50000,
}

#[derive(Serialize)]
pub struct Result<T>
where
    T: Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<Code>,
}

pub type JsonResult<T> = status::Custom<Json<Result<T>>>;

pub fn result<T>(
    status: Status,
    code: Option<Code>,
    msg: Option<String>,
    data: Option<T>,
) -> JsonResult<T>
where
    T: Serialize,
{
    let res = Result {
        data: data,
        msg: msg,
        code: code,
    };
    status::Custom(status, Json(res))
}

pub fn ok<T>(data: T) -> JsonResult<T>
where
    T: Serialize,
{
    result(Status::Ok, Some(Code::Ok), None, Some(data))
}

pub fn error<T>(err: impl Error) -> JsonResult<T>
where
    T: Serialize,
{
    result(
        Status::InternalServerError,
        Some(Code::Err),
        Some(err.to_string()),
        None,
    )
}
