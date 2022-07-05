use crate::common::db::conn;
use crate::common::error::ToError;
use crate::common::redis_util;
use crate::common::BasicResult;
use crate::common::Pagination;
use crate::dao::user as user_dao;
use crate::dao::user::UserType;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::__Deref;
use redis::ToRedisArgs;
use redis_encoding_derive::ToRedisArgs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::time;
use uuid::Uuid;

struct Token {
    secret: String,
    algorithm: Algorithm,
    duration: usize,
}

pub struct Service {
    token: Token,
}

impl Service {
    pub fn new() -> Self {
        Self {
            token: Token {
                secret: "secret".to_string(),
                algorithm: Algorithm::HS512,
                duration: 60 * 60 * 24,
            },
        }
    }
}

impl Service {
    fn hash_password(password: &str, salt: &str) -> String {
        let mut hasher = sha2::Sha512::new();
        hasher.update(password.as_bytes());
        hasher.update(b"$");
        hasher.update(salt.as_bytes());
        base64::encode(&hasher.finalize())
    }

    fn salt() -> String {
        Uuid::new_v4().to_string()
    }

    fn check_pwd(pwd: &str, salt: &str, pwd_hashed: &str) -> BasicResult<()> {
        let pwd = Self::hash_password(pwd, salt);
        let pwd = pwd.deref();

        if pwd != pwd_hashed {
            return Err("invalid password".to_basic_error());
        }

        Ok(())
    }

    fn validate_email(email: &str) -> BasicResult<()> {
        let reg = Regex::new(r#"\w[-\w.+]*@([A-Za-z0-9][-A-Za-z0-9]+\.)+[A-Za-z]{2,14}"#)?;
        if !reg.is_match(email) {
            return Err("invalid email".to_validation_error());
        }
        Ok(())
    }

    fn validate_mobile(mobile: &str) -> BasicResult<()> {
        let reg = Regex::new(r#"0?(13|14|15|17|18|19)[0-9]{9}"#)?;
        if !reg.is_match(mobile) {
            return Err("invalid mobile".to_validation_error());
        }
        Ok(())
    }

    fn validate_pwd(pwd: &str) -> BasicResult<()> {
        let reg = Regex::new(r#"^[a-zA-Z]{1}\w{5,17}$"#)?; //6位字母+数字,字母开头
        if !reg.is_match(pwd) {
            return Err(
                "invalid passowrd: length>=6, a-z and 0-9 is demanded".to_validation_error()
            );
        }
        Ok(())
    }
}

impl Service {
    pub async fn query(&self, page: &Pagination) -> BasicResult<(Vec<user_dao::User>, i64)> {
        let e1 = &conn().await?;
        let e2 = &conn().await?;
        let (data, count) = tokio::join!(
            user_dao::Dao::new(e1).query(page),
            user_dao::Dao::new(e2).count()
        );
        let data = data?;
        let count = count?;

        Ok((data, count))
    }

    pub async fn get(&self, id: i64) -> BasicResult<user_dao::User> {
        let res = user_dao::Dao::new(&conn().await?).get(id).await?;
        Ok(res)
    }

    pub async fn get_by_email(&self, email: &str) -> BasicResult<user_dao::User> {
        let res = user_dao::Dao::new(&conn().await?)
            .get_by_email(email)
            .await?;
        Ok(res)
    }

    pub async fn insert(
        &self,
        email: &str,
        pwd: &str,
        name: Option<&str>,
        mobile: Option<&str>,
    ) -> BasicResult<i64> {
        Self::validate_email(email)?;
        if let Some(x) = mobile {
            Self::validate_mobile(x)?;
        }
        Self::validate_pwd(pwd)?;
        let salt = Self::salt();
        let pwd = Self::hash_password(pwd, &salt);
        let insert_id = user_dao::Dao::new(&conn().await?)
            .insert(email, &salt, &pwd, name, mobile)
            .await?;

        Ok(insert_id)
    }

    pub async fn delete(&self, ids: &[i64]) -> BasicResult<u64> {
        let res = user_dao::Dao::new(&conn().await?).delete(ids).await?;

        Ok(res)
    }

    pub async fn update_pwd(&self, email: &str, old_pwd: &str, new_pwd: &str) -> BasicResult<u64> {
        Self::validate_pwd(old_pwd)?;
        Self::validate_pwd(new_pwd)?;
        if old_pwd == new_pwd {
            return Err(
                "new password can not be the same as old password !!!".to_validation_error()
            );
        }
        let user = self.get_by_email(email).await?;

        Self::check_pwd(old_pwd, &user.salt, &user.pwd.unwrap_or_default())?;

        let salt = Self::salt();
        let pwd = Self::hash_password(new_pwd, &salt);
        let res = user_dao::Dao::new(&conn().await?)
            .update_pwd(user.id, &salt, &pwd)
            .await?;

        Ok(res)
    }

    pub async fn login(&self, email: &str, pwd: &str) -> BasicResult<String> {
        let user = self.get_by_email(email).await?;

        Self::check_pwd(pwd, &user.salt, &user.pwd.unwrap_or_default())?;

        user_dao::Dao::new(&conn().await?)
            .update_laston(user.id)
            .await?;

        let header = Header::new(self.token.algorithm);

        let iat = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)?
            .as_secs() as usize;
        let exp = iat + self.token.duration;

        let claims = Claims {
            aud: user.email.to_owned(),
            exp: exp,
            iat: iat,
        };
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.token.secret.as_ref()),
        )
        .map_err(|err| {
            log::error!("encode token err: {}", err.to_string());
            err
        })?;

        redis_util::set_ex(
            user.email.clone(),
            CurrentUser {
                id: user.id,
                r#type: user.r#type,
                email: user.email,
                name: user.name,
                mobile: user.mobile,
                laston: user.laston.map(|x| x.to_string()),
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.map(|x| x.to_string()),
            },
            self.token.duration,
        )
        .await?;

        return Ok(token);
    }

    pub async fn check(&self, token: &str) -> BasicResult<CurrentUser> {
        let validation = Validation::new(self.token.algorithm);
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.token.secret.as_ref()),
            &validation,
        )
        .map_err(|err| {
            log::error!("decode token failed: {}", err.to_string());
            err
        })?;
        let email = claims.claims.aud;
        let current_user = redis_util::get::<_, CurrentUser>(email)
            .await
            .map_err(|err| {
                log::error!("get user from redis failed: {}", err.to_string());
                err
            })?;
        Ok(current_user)
    }
}

#[derive(Debug, Serialize, Deserialize, ToRedisArgs)]
pub struct CurrentUser {
    pub id: i64,
    pub r#type: UserType,
    pub email: String,
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub laston: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    // sub: Option<String>, // Optional. Subject (whom token refers to)
    iat: usize, // Optional. Issued at (as UTC timestamp)
                // iss: String, // Optional. Issuer
                // nbf: usize, // Optional. Not Before (as UTC timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_query() -> BasicResult<()> {
        let (res, count) = Service::new()
            .query(&Pagination { index: 1, size: 2 })
            .await?;

        println!("query count: {}", count);

        println!("query result: {:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_get() -> BasicResult<()> {
        let res = Service::new().get(1).await?;

        println!("get result: {:?}", res);

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_insert() -> BasicResult<()> {
        let insert_id = Service::new()
            .insert("test@qq.com", "123", Some("你好"), Some("13156781111"))
            .await?;

        println!("insert_id: {}", insert_id);
        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn test_insert_invalid_email() {
        let insert_id = Service::new()
            .insert("testqq.com", "123", Some("你好"), Some("13156781111"))
            .await
            .unwrap();

        println!("insert_id: {}", insert_id);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_insert_invalid_mobile() {
        let insert_id = Service::new()
            .insert("test@qq.com", "123", Some("你好"), Some("111333444"))
            .await
            .unwrap();

        println!("insert_id: {}", insert_id);
    }

    // #[tokio::test]
    // async fn test_delete() -> SqlResult<()> {
    //     let res = Dao::new(&conn().await?).delete(&[3, 5]).await?;

    //     println!("rows_affected: {}", res);

    //     Ok(())
    // }

    #[tokio::test]
    async fn test_update_pwd() -> BasicResult<()> {
        let res = Service::new()
            .update_pwd("453017973@qq.com", "123", "123")
            .await?;

        println!("rows_affected: {}", res);

        Ok(())
    }
}
