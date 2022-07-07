use super::guard::Auth;
use crate::controller::common::*;
use crate::service::user as user_service;
use crate::{common::Pagination, dao::user::UserType};
use rocket::serde::{json::Json, Deserialize, Serialize};

// #[derive(Serialize, Component)]
#[derive(Serialize)]
pub struct User {
    id: i64,
    r#type: UserType,
    email: String,
    name: Option<String>,
    mobile: Option<String>,
    laston: Option<String>,
    created_at: String,
    updated_at: Option<String>,
}

#[get("/query?<page..>")]
pub async fn query(page: Pagination) -> JsonResult<Vec<User>> {
    match user_service::Service::new().query(&page).await {
        Ok(data) => {
            let mut res = vec![];

            data.0.into_iter().for_each(|x| {
                res.push(User {
                    id: x.id,
                    r#type: x.r#type,
                    email: x.email,
                    name: x.name,
                    mobile: x.mobile,
                    laston: x.laston.map(|x| x.to_string()),
                    created_at: x.created_at.to_string(),
                    updated_at: x.updated_at.map(|x| x.to_string()),
                })
            });
            ok_with_data_pagination(res, data.1, page)
        }
        Err(err) => error(err),
    }
}

#[get("/<id>")]
pub async fn get(id: i64, _auth: Auth) -> JsonResult<User> {
    match user_service::Service::new().get(id).await {
        Ok(x) => {
            let data = User {
                id: x.id,
                r#type: x.r#type,
                email: x.email,
                name: x.name,
                mobile: x.mobile,
                laston: x.laston.map(|x| x.to_string()),
                created_at: x.created_at.to_string(),
                updated_at: x.updated_at.map(|x| x.to_string()),
            };
            ok_with_data(data)
        }
        Err(err) => error(err),
    }
}

#[get("/check")]
pub async fn check(_auth: Auth) -> JsonResult<user_service::CurrentUser> {
    ok_with_data(_auth.current_user)
}

#[derive(Deserialize)]
pub struct UserInsertReq<'a> {
    email: &'a str,
    pwd: &'a str,
    mobile: Option<&'a str>,
    name: Option<&'a str>,
}

#[post("/insert", data = "<req>")]
pub async fn insert(req: Json<UserInsertReq<'_>>) -> JsonResult<i64> {
    match user_service::Service::new()
        .insert(req.email, req.pwd, req.name, req.mobile)
        .await
    {
        Ok(id) => ok_with_data(id),
        Err(err) => error(err),
    }
}

#[derive(Deserialize)]
pub struct LoginReq<'a> {
    email: &'a str,
    pwd: &'a str,
}

#[post("/login", data = "<req>")]
pub async fn login(req: Json<LoginReq<'_>>) -> JsonResult<String> {
    match user_service::Service::new().login(req.email, req.pwd).await {
        Ok(token) => ok_with_data(token),
        Err(err) => error(err),
    }
}

#[derive(Deserialize)]
pub struct UpdatePasswordReq<'a> {
    email: &'a str,
    old_pwd: &'a str,
    new_pwd: &'a str,
}

#[put("/update_pwd", data = "<req>")]
pub async fn update_pwd(req: Json<UpdatePasswordReq<'_>>) -> JsonResult<()> {
    match user_service::Service::new()
        .update_pwd(req.email, req.old_pwd, req.new_pwd)
        .await
    {
        Ok(_) => ok(),
        Err(err) => error(err),
    }
}

// #[put("/user/update", data = "<req>")]
// pub async fn update(req: Json<UserUpdateReq<'_>>) -> JsonResult<u64> {
//     match user_service::Service::new().update(req.id, req.name).await {
//         Ok(data) => ok_with_data(data),
//         Err(err) => error(err),
//     }
// }

// #[derive(Deserialize)]
// pub struct UserInsertReq<'a> {
//     email: &'a str,
//     role_id: i64,
// }

// #[post("/user/delete", data = "<req>")]
// pub async fn delete(req: Json<ID>) -> JsonResult<u64> {
//     match user_service::Service::new().delete(req.id).await {
//         Ok(data) => ok_with_data(data),
//         Err(err) => error(err),
//     }
// }
