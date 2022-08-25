use crate::repositories::courses_repository;
use actix_web::{get, post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]

pub struct CourseFromClient {
    pub prof: String,
    pub schedule: i64,
    pub theme: String,
    pub address: String,
    pub level: String,
    pub comments: String,
}

#[derive(Deserialize)]
pub struct Query {
    page: u16,
}

#[get("/courses")]
pub async fn get_courses(pagination: web::Query<Query>) -> Result<HttpResponse, Error> {
    let start = Instant::now();

    let courses = courses_repository::get_courses(pagination.page);
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
