use crate::claim::*;
use crate::repositories::students_repository::*;
use crate::security::password_manager::*;
use crate::{claim::decode_jwt, repositories::students_repository};
use actix_web::{error, get, post, web, Error, HttpResponse, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
// use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use derive_more::Error;
use email_address::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize, Debug)]
pub struct UserInput {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UserInputForLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UserRegistration {
    pub course_uuid: String,
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

#[derive(Serialize, Deserialize)]
pub struct StudentUpdate {
    pub name: String,
    pub grade: String,
    pub photo: String,
    pub availability: String,
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

    if !EmailAddress::is_valid(&user_info.email) {
        return Ok(HttpResponse::BadRequest().json(CustomError {
            message: "You should provide a valid email address.",
            code: 45687,
        }));
    }

    if is_user_exists(&user_info.email) {
        return Ok(HttpResponse::BadRequest().json(CustomError {
            message: "User already exists.",
            code: 89045,
        }));
    }

    let pass_h = hash_password(&user_info.password);

    let user_permissions = vec!["OP_GET_SECURED_INFO".to_string(), "ROLE_USER".to_string()];

    let user_id = insert_new_user(
        &user_info.email,
        &user_info.name,
        &pass_h,
        (*user_permissions).to_vec(),
    );

    let claims = Claims::new(
        &user_id,
        &user_info.name,
        &user_info.email,
        &user_permissions,
    );
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
pub async fn login(info: web::Json<UserInputForLogin>) -> Result<HttpResponse, Error> {
    let user_info = info.into_inner();

    let start = Instant::now();
    let user_in_database = get_user(&user_info.email);

    let duration = start.elapsed();
    println!(
        "Time elapsed in &get_user(&user_info.username) is: {:?}",
        duration
    );
    //     HttpResponse::
    if user_in_database.is_empty() {
        Ok(HttpResponse::Unauthorized().json(CustomError {
            code: 16873154,
            message: "User doesn't exists",
        }))
    } else {
        let user_from_db = &user_in_database[0];

        match verify_password(
            user_info.password.to_string(),
            user_from_db.password.to_string(),
        ) {
            true => {
                let start = Instant::now();

                let claims = Claims::new(
                    &user_from_db.guid,
                    &user_from_db.name,
                    &user_info.email,
                    &user_from_db.user_permissions,
                );
                let jwt = create_jwt(claims)?;
                let duration = start.elapsed();

                let expiration_time = (Utc::now()
                    + Duration::hours(
                        dotenv!("TOKEN_DURATION_TIME_HOURS").parse::<i64>().unwrap(),
                    ))
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

#[post("/subscribe")]
pub async fn course_registration(
    credentials: BearerAuth,
    info: web::Json<UserRegistration>,
) -> Result<HttpResponse, Error> {
    let user_input = info.into_inner();
    let token_decoded = decode_jwt(credentials.token()).unwrap();

    if is_user_alredy_subscribe(&token_decoded.user_id, &user_input.course_uuid) {
        return Ok(HttpResponse::BadRequest().json(CustomError {
            message: "User already subscribed.",
            code: 39513,
        }));
    }

    subscribe_to_a_course(&token_decoded.user_id, &user_input.course_uuid);

    Ok(HttpResponse::Created().json("success"))
}

#[post("/unsubscribe")]
pub async fn course_deregistration(
    credentials: BearerAuth,
    info: web::Json<UserRegistration>,
) -> Result<HttpResponse, Error> {
    let user_input = info.into_inner();
    let token_decoded = decode_jwt(credentials.token()).unwrap();

    if !is_user_alredy_subscribe(&token_decoded.user_id, &user_input.course_uuid) {
        return Ok(HttpResponse::BadRequest().json(CustomError {
            message: "User is not subscribed.",
            code: 39513,
        }));
    }

    unsubscribe_to_a_course(&token_decoded.user_id, &user_input.course_uuid);

    Ok(HttpResponse::Ok().json("success"))
}

#[post("/update-student")]
pub async fn update_student(
    credentials: BearerAuth,
    info: web::Json<StudentUpdate>,
) -> Result<HttpResponse, Error> {
    let user_input = info.into_inner();
    let token_decoded = decode_jwt(credentials.token()).unwrap();

    students_repository::update_student(&token_decoded.user_id, user_input);

    Ok(HttpResponse::NoContent().json("success"))
}

#[get("/me")]
pub async fn me(credentials: BearerAuth) -> Result<HttpResponse, Error> {
    let token_decoded = decode_jwt(credentials.token()).unwrap();

    let me = students_repository::get_me(&token_decoded.user_id);

    Ok(HttpResponse::Ok().json(me))
}
