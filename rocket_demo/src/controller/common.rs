use crate::common::{error, Pagination};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Serialize};

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
pub struct ResultData<T>
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
    page: Option<Page>,
}
#[derive(Serialize)]
pub struct Page {
    pub total: i64,
    pub index: i64,
    pub size: i64,
}

pub type JsonResult<T> = status::Custom<Json<ResultData<T>>>;

pub fn result<T>(
    status: Status,
    code: Option<Code>,
    msg: Option<String>,
    data: Option<T>,
    page: Option<Page>,
) -> JsonResult<T>
where
    T: Serialize,
{
    let res = ResultData {
        data,
        msg,
        code,
        page,
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

pub fn ok_with_data_pagination<T>(data: T, total: i64, p: Pagination) -> JsonResult<T>
where
    T: Serialize,
{
    result(
        Status::Ok,
        Some(Code::Ok),
        None,
        Some(data),
        Some(Page {
            total: total,
            index: p.index,
            size: p.size,
        }),
    )
}

pub fn error<T>(err: error::ErrorKind) -> JsonResult<T>
where
    T: Serialize,
{
    match err {
        error::ErrorKind::ValidationError(msg) => {
            result(Status::BadRequest, Some(Code::Err), Some(msg), None, None)
        }
        _ => result(
            Status::InternalServerError,
            Some(Code::Err),
            Some(err.to_string()),
            None,
            None,
        ),
    }
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
