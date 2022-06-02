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
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<i64>,
}

pub type JsonResult<T> = status::Custom<Json<Result<T>>>;

pub fn result<T>(
    status: Status,
    code: Option<Code>,
    msg: Option<String>,
    data: Option<T>,
    total: Option<i64>,
) -> JsonResult<T>
where
    T: Serialize,
{
    let res = Result {
        data,
        msg,
        code,
        total,
    };
    status::Custom(status, Json(res))
}

pub fn ok<'a>() -> JsonResult<()> {
    result(Status::Ok, Some(Code::Ok), None, None, None)
}

pub fn ok_with_msg(msg: &str) -> JsonResult<()> {
    result(
        Status::Ok,
        Some(Code::Ok),
        Some(msg.to_string()),
        None,
        None,
    )
}

pub fn ok_with_data<'a, T>(data: T) -> JsonResult<T>
where
    T: Serialize,
{
    result(Status::Ok, Some(Code::Ok), None, Some(data), None)
}

pub fn ok_with_data_pagination<T>(data: T, total: i64) -> JsonResult<T>
where
    T: Serialize,
{
    result(Status::Ok, Some(Code::Ok), None, Some(data), Some(total))
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
        None,
    )
}

pub fn error_with_msg<T>(msg: &str) -> JsonResult<T>
where
    T: Serialize,
{
    result(
        Status::InternalServerError,
        Some(Code::Err),
        Some(msg.to_string()),
        None,
        None,
    )
}
