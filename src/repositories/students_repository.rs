use crate::routes::students::StudentUpdate;
use rusqlite::Connection;
use serde::Serialize;
use uuid::Uuid;

const DB_PATH: &str = dotenv!("DATABASE_PATH");

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub guid: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]

pub struct FullUser {
    pub guid: String,
    pub name: Option<String>,
    pub email: String,
    pub grade: Option<String>,
    pub photo: Option<String>,
    pub availability: Option<String>,
}

pub fn connect() -> Connection {
    Connection::open(DB_PATH).unwrap()
}

pub fn insert_new_user(email: &str, password: &str, _user_permission: Vec<String>) -> String {
    let conn = connect();

    let uuid = Uuid::new_v4();

    match conn.execute(
        "INSERT INTO students (guid, email, password) values (?1, ?2, ?3);",
        &[&uuid.to_string(), email, password],
    ) {
        Ok(inserted) => println!("{} rows were inserted", inserted),
        Err(err) => println!("insert failed: {}", err),
    }

    conn.query_row(
        "SELECT guid FROM students WHERE guid = ?1;",
        [uuid.to_string()],
        |row| row.get(0),
    )
    .unwrap()
}

pub fn is_user_exists(email: &str) -> bool {
    let conn = connect();

    !matches!(
        conn.query_row(
            "SELECT email FROM students WHERE email = ?1;",
            &[email],
            |row| row.get::<usize, String>(0),
        ),
        Err(_)
    )
}

// pub fn update_user() -> bool {}

pub fn get_user(username: &str) -> Vec<User> {
    let conn = connect();

    let mut stmt = conn
        .prepare("SELECT guid, email, password FROM students WHERE email = ?1")
        .unwrap();

    let users = stmt
        .query_map(&[username], |row| {
            Ok(User {
                guid: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .unwrap();

    let mut to_return: Vec<User> = vec![];

    for user in users {
        to_return.push(user.unwrap());
    }

    to_return
}

pub fn is_user_alredy_subscribe(student_uui: &str, course_uuid: &str) -> bool {
    let conn = connect();

    // match conn.query_row(
    //     "SELECT guid FROM courses_students WHERE id_student = ?1 AND course_uuid = ?2",
    //     &[student_uui, course_uuid],
    //     |row| row.get::<usize, String>(0),
    // ){
    //     Err(_) => false,
    //     _ => true
    //  }

    matches!(
        conn.query_row(
            "SELECT guid FROM courses_students WHERE id_student = ?1 AND course_uuid = ?2",
            &[student_uui, course_uuid],
            |row| row.get::<usize, String>(0),
        ),
        Err(_)
    )
}

pub fn subscribe_to_a_course(student_uui: &str, course_uuid: &str) {
    let conn = connect();

    let uuid = Uuid::new_v4();

    match conn.execute(
        "INSERT INTO courses_students (guid, id_student, id_course) values (?1, ?2, ?3);",
        &[&uuid.to_string(), student_uui, course_uuid],
    ) {
        Ok(inserted) => println!("{} rows were inserted", inserted),
        Err(err) => println!("insert failed: {}", err),
    }
}

pub fn update_student(student_uuid: &str, student_update: StudentUpdate) -> usize {
    let conn = connect();

    let mut stmt = conn.prepare(
        "UPDATE students SET name = ?1, password = ?2, grade = ?3, photo = ?4, availability = ?5 WHERE guid = ?6"
    ).unwrap();

    let StudentUpdate {
        name,
        password,
        grade,
        photo,
        availability,
    } = student_update;

    stmt.execute([
        &name,
        &password,
        &grade,
        &photo,
        &availability,
        student_uuid,
    ])
    .unwrap()
}
