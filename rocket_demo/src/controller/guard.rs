use crate::common::error::{self, ErrorKind};
use crate::common::BasicResult;
use crate::service::user as user_service;
use rocket::{
    fairing::Fairing,
    request::{FromRequest, Outcome},
};

pub struct Auth {
    pub token: String,
    pub current_user: user_service::CurrentUser,
}

impl<'a> Auth {
    async fn valid(token: &str) -> BasicResult<Self> {
        let current_user = user_service::Service::new()
            .check(token.trim_start_matches("Bearer "))
            .await?;
        Ok(Auth {
            token: token.to_string(),
            current_user,
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ErrorKind;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let authentication = request.headers().get_one("Authorization");
        match authentication {
            Some(token) => match Self::valid(token).await {
                Ok(auth) => Outcome::Success(auth),
                Err(err) => Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    error::basic(&err.to_string()),
                )),
            },
            None => Outcome::Failure((
                rocket::http::Status::Unauthorized,
                error::basic("token missing"),
            )),
        }
    }
}

#[rocket::async_trait]
impl Fairing for Auth {
    fn info(&self) -> rocket::fairing::Info {
        todo!()
    }
}
