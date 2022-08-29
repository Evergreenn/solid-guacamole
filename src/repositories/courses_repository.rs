use crate::routes::courses;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

use super::students_repository;

const DB_PATH: &str = dotenv!("DATABASE_PATH");

#[derive(Debug, PartialEq, Eq, derive_more::Error, Serialize, Deserialize)]
pub struct CourseWithJoin {
    pub guid: String,
    pub prof: String,
    pub schedule: i64,
    pub theme: String,
    pub address: String,
    pub level: String,
    pub comments: String,
    pub subscriber_count: i32,
    pub insert_date: NaiveDateTime,
}


impl Display for CourseWithJoin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {}, {}, {}, {}, {})",
            self.guid,
            self.prof,
            self.schedule,
            self.theme,
            self.address,
            self.level,
            self.comments,
            self.insert_date,
            self.subscriber_count,
        )
    }
}


pub fn get_users_subscribed(course_guid: &str) -> Vec<students_repository::FullUser> {

        let conn = connect();
        let mut stmt = conn.prepare("SELECT s.guid, s.name, s.email, s.grade, s.photo, s.availablity FROM students s INNER JOIN courses_students cs ON cs.id_student = s.guid WHERE cs.id_course = ?1 GROUP BY s.guid").unwrap();

        let users_subs = stmt.query_map(params![course_guid], |row| {
            Ok(students_repository::FullUser {
                guid: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                email: row.get(2).unwrap(),
                grade: row.get(3).unwrap(),
                photo: row.get(4).unwrap(),
                availability: row.get(5).unwrap()
            })
        }).unwrap();

       users_subs 
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<students_repository::FullUser>>()
    }


fn connect() -> Connection {
    Connection::open(DB_PATH).unwrap()
}

pub fn get_courses(mut page: u16) -> Vec<CourseWithJoin> {
    let conn = connect();

    if page < 1u16 {
        page = 1u16;
    }

    let offset = (page -1) * 5u16;
    let limit = 5u16;
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let now_in_timestamp = since_the_epoch.as_secs();

    let mut stmt = conn.prepare("SELECT c.*, COUNT(cs.guid) as nb_subscribers FROM courses c LEFT JOIN courses_students cs ON  c.guid = cs.id_course WHERE schedule_date > ?1 LIMIT ?2 OFFSET ?3").unwrap();

    let course_iter = stmt
        .query_map(params![now_in_timestamp, limit, offset], |row| {
            Ok(CourseWithJoin {
                guid: row.get(0).unwrap(),
                prof: row.get(1).unwrap(),
                schedule: row.get(2).unwrap(),
                theme: row.get(3).unwrap(),
                address: row.get(4).unwrap(),
                level: row.get(5).unwrap(),
                comments: row.get(6).unwrap(),
                insert_date: row.get(7).unwrap(),
                subscriber_count: row.get(8).unwrap(),
            })
        })
        .unwrap();

    course_iter
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<CourseWithJoin>>()
}


pub fn insert_course(course: courses::CourseFromClient) -> i64 {
    let conn = connect();

    let uuid = Uuid::new_v4();

    match conn.execute(
        "INSERT INTO courses (guid, prof, schedule_date, theme, address, level, comments) values (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
        params![&uuid.to_string().to_owned(), &course.prof, &course.schedule, &course.theme, &course.address, &course.level, &course.comments],
    ) {
        Ok(inserted) => println!("{} rows were inserted", inserted),
        Err(err) => println!("insert failed: {}", err),
    }

    conn.last_insert_rowid()
}