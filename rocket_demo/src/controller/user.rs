use crate::controller::common::*;
use crate::service::user as user_service;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    id: i64,
    email: String,
    role_name: String,
}

#[get("/user/query")]
pub async fn query() -> JsonResult<Vec<User>> {
    match user_service::Service::new().query().await {
        Ok(data) => {
            let mut res = vec![];

            data.iter().for_each(|x| {
                res.push(User {
                    id: x.id,
                    email: x.email.clone(),
                    role_name: x.role_name.clone(),
                })
            });
            ok(res)
        }
        Err(err) => error(err),
    }
}
