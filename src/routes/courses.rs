use crate::claim::decode_jwt;
use crate::claim::*;
use crate::repositories::courses_repository;
use crate::security::password_manager::*;
use actix_web::{error, get, post, web, Error, HttpResponse, Responder, Result};
// use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc, NaiveDateTime, DateTime};
use derive_more::Error;
use email_address::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

// #[derive(Serialize, Deserialize, Debug)]
// struct CoursesRespond {
//     pub courses: Vec<courses_repository::Course>
// }
#[derive(Debug, Serialize, Deserialize)]

pub struct CourseFromClient {
    pub prof: String,
    pub schedule: i64,
    pub theme: String,
    pub address: String,
    pub level: String,
    pub comments: String,
}

#[get("/courses")]
pub async fn get_courses() -> Result<HttpResponse, Error> {
    let start = Instant::now();

    let courses = courses_repository::get_courses();
    let duration = start.elapsed();
    dbg!("Time elapsed in get_courses is: {:?}", duration);

    Ok(HttpResponse::Ok().json(courses))
}

#[post("/add-courses")]
pub async fn add_course(
    info: web::Json<CourseFromClient>,
) -> Result<HttpResponse, Error> {
    let course = info.into_inner();

    let start = Instant::now();

    let courses = courses_repository::insert_course(course);
    let duration = start.elapsed();
    dbg!("Time elapsed in add_course is: {:?}", duration);

    Ok(HttpResponse::Created().json(courses))
}
