use crate::routes::courses;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

const DB_PATH: &str = dotenv!("DATABASE_PATH");

#[derive(Debug, PartialEq, Eq, derive_more::Error, Serialize, Deserialize)]
pub struct Course {
    pub guid: String,
    pub prof: String,
    pub schedule: i64,
    pub theme: String,
    pub address: String,
    pub level: String,
    pub comments: String,
    pub insert_date: NaiveDateTime,
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {}, {}, {}, {})",
            self.guid,
            self.prof,
            self.schedule,
            self.theme,
            self.address,
            self.level,
            self.comments,
            self.insert_date,
        )
    }
}

fn connect() -> Connection {
    Connection::open(DB_PATH).unwrap()
}

pub fn get_courses(mut page: u16) -> Vec<Course> {
    let conn = connect();

    if page < 1u16 {
        page = 1u16;
    }

    let offset = (page -1) * 5u16;
    let limit = 5u16;

    let mut stmt = conn.prepare("SELECT * FROM courses LIMIT ?1 OFFSET ?2").unwrap();

    let course_iter = stmt
        .query_map([limit, offset], |row| {
            Ok(Course {
                guid: row.get(0).unwrap(),
                prof: row.get(1).unwrap(),
                schedule: row.get(2).unwrap(),
                theme: row.get(3).unwrap(),
                address: row.get(4).unwrap(),
                level: row.get(5).unwrap(),
                comments: row.get(6).unwrap(),
                insert_date: row.get(7).unwrap(),
            })
        })
        .unwrap();

    course_iter
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<Course>>()
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
