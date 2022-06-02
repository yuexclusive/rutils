use crate::controller::common::*;
use crate::model::common::{Pagination, ID};
use crate::service::user as user_service;
use rocket::serde::{json::Json, Deserialize, Serialize};

// #[derive(Serialize, Component)]
#[derive(Serialize)]
pub struct User {
    id: i64,
    email: String,
    role_name: String,
}

#[get("/user/query?<page..>")]
pub async fn query(page: Pagination) -> JsonResult<Vec<User>> {
    match user_service::Service::new().query(&page).await {
        Ok(data) => {
            let mut res = vec![];

            data.0.iter().for_each(|x| {
                res.push(User {
                    id: x.id,
                    email: x.email.clone(),
                    role_name: x.role_name.clone(),
                })
            });
            ok_with_data_pagination(res, data.1)
        }
        Err(err) => error(err),
    }
}

#[derive(Deserialize)]
pub struct UserUpdateReq<'a> {
    id: i64,
    name: &'a str,
}

#[put("/user/update", data = "<req>")]
pub async fn update(req: Json<UserUpdateReq<'_>>) -> JsonResult<u64> {
    match user_service::Service::new().update(req.id, req.name).await {
        Ok(data) => ok_with_data(data),
        Err(err) => error(err),
    }
}

#[derive(Deserialize)]
pub struct UserInsertReq<'a> {
    email: &'a str,
    role_id: i64,
}

#[post("/user/insert", data = "<req>")]
pub async fn insert(req: Json<UserInsertReq<'_>>) -> JsonResult<u64> {
    match user_service::Service::new()
        .insert(req.email, req.role_id)
        .await
    {
        Ok(data) => ok_with_data(data),
        Err(err) => error(err),
    }
}

#[post("/user/delete", data = "<req>")]
pub async fn delete(req: Json<ID>) -> JsonResult<u64> {
    match user_service::Service::new().delete(req.id).await {
        Ok(data) => ok_with_data(data),
        Err(err) => error(err),
    }
}
