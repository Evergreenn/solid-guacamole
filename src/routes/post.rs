use crate::claim::decode_jwt;
use crate::claim::*;
use crate::repositories::user_repository::*;
use crate::security::password_manager::*;
use actix_web::{error, post, web, Error, HttpResponse, Responder, Result};
// use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use derive_more::Error;
use serde::{Deserialize, Serialize};
use email_address::*;
use std::time::Instant;

#[derive(Deserialize, Debug)]
pub struct UserInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResult {
    pub jwt: String,
    pub expiration_time: i64,
}

#[derive(Debug, Error, Serialize)]
struct CustomError {
    message: &'static str,
    code: usize,
}

impl error::ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(self.to_string())
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{\"code\" : {}, \"message\": \"{}\" }}",
            self.code, self.message
        )
    }
}

#[post("/register")]
pub async fn create_token(info: web::Json<UserInput>) -> Result<HttpResponse, Error> {
    let user_info = info.into_inner();

    if !EmailAddress::is_valid(&user_info.email){
        return Ok(HttpResponse::BadRequest().json(CustomError {
            message: "You should provide a valid email address.",
            code: 045687,
        }));
    }

    if is_user_exists(&user_info.email) {
        return Ok(HttpResponse::BadRequest().json(CustomError {
            message: "User already exists.",
            code: 089046,
        }));
    }

    let pass_h = hash_password(&user_info.password);

    let user_permissions = vec!["OP_GET_SECURED_INFO".to_string(), "ROLE_USER".to_string()];

    let user_id = insert_new_user(&user_info.email, &pass_h, (&*user_permissions).to_vec());

    let claims = Claims::new(&user_id, &user_info.email, user_permissions);
    let jwt = create_jwt(claims)?;
    let expiration_time = (Utc::now()
        + Duration::hours(dotenv!("TOKEN_DURATION_TIME_HOURS").parse::<i64>().unwrap()))
    .timestamp();

    Ok(HttpResponse::Created().json(LoginResult {
        jwt,
        expiration_time,
    }))
}

#[post("/login")]
pub async fn login(info: web::Json<UserInput>) -> Result<HttpResponse, Error> {
    let user_info = info.into_inner();

    let start = Instant::now();
    let user_in_database = get_user(&user_info.email);

    let duration = start.elapsed();
    println!("Time elapsed in &get_user(&user_info.username) is: {:?}", duration);
    //     HttpResponse::
    if user_in_database.len() == 0 {
        Ok(HttpResponse::Unauthorized().json(CustomError {
            code: 16873154,
            message: "User doesn't exists",
        }))
    }else {

        let user_from_db = &user_in_database[0];

        match verify_password(
            user_info.password.to_string(),
            user_from_db.password.to_string(),
        ) {
            true => {
                let user_permissions = vec!["OP_GET_SECURED_INFO".to_string(), "ROLE_USER".to_string()];
                let start = Instant::now();


                let claims = Claims::new(
                    &user_from_db.guid,
                    &user_info.email,
                    user_permissions,
                );
                let jwt = create_jwt(claims)?;
                let duration = start.elapsed();

                let expiration_time = (Utc::now()
                + Duration::hours(dotenv!("TOKEN_DURATION_TIME_HOURS").parse::<i64>().unwrap()))
                .timestamp();
                println!("Time elapsed in create jwt is: {:?}", duration);
                Ok(HttpResponse::Ok().json(LoginResult {
                    jwt,
                    expiration_time,
                }))
            }
            false => Ok(HttpResponse::Unauthorized().json(CustomError {
                code: 16873154,
                message: "Wrong Password",
            })),
        }

    }
}
