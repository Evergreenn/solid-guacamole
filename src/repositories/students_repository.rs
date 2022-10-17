use crate::routes::students::StudentUpdate;
use rusqlite::Connection;
use serde::Serialize;
use uuid::Uuid;

const DB_PATH: &str = dotenv!("DATABASE_PATH");

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub guid: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub user_permissions: Vec<String>,
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

pub fn insert_new_user(
    email: &str,
    name: &str,
    password: &str,
    user_permission: Vec<String>,
) -> String {
    let conn = connect();

    let uuid = Uuid::new_v4();
    let user_permissions_joined = user_permission.join(",");

    match conn.execute(
        "INSERT INTO students (guid, email, name, password, user_permissions) values (?1, ?2, ?3, ?4, ?5);",
        &[&uuid.to_string(), email, name, password, &user_permissions_joined],
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
pub fn get_me(user_id: &str) -> Vec<FullUser> {
    let conn = connect();

    let mut stmt = conn
        .prepare("SELECT email, name, grade, photo, availability FROM students WHERE guid = ?1")
        .unwrap();

    let users = stmt
        .query_map(&[user_id], |row| {
            Ok(FullUser {
                guid: user_id.to_string(),
                email: row.get(0)?,
                name: row.get(1)?,
                grade: row.get(2)?,
                photo: row.get(3)?,
                availability: row.get(4)?,
            })
        })
        .unwrap();

    //TODO: Change ths it doesn´t need to be a loop
    let mut to_return: Vec<FullUser> = vec![];

    for user in users {
        to_return.push(user.unwrap());
    }

    to_return
}

pub fn get_user(username: &str) -> Vec<User> {
    let conn = connect();

    let mut stmt = conn
        .prepare(
            "SELECT guid, email, name, password, user_permissions FROM students WHERE email = ?1",
        )
        .unwrap();

    let users = stmt
        .query_map(&[username], |row| {
            let collected: String = row.get(4).unwrap();
            let vectorized = collected.split(",").map(|s| s.to_string()).collect();

            Ok(User {
                guid: row.get(0)?,
                email: row.get(1)?,
                name: row.get(2)?,
                password: row.get(3)?,
                user_permissions: vectorized,
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

    match conn.query_row(
        "SELECT guid FROM courses_students WHERE id_student = ?1 AND id_course = ?2",
        &[student_uui, course_uuid],
        |row| row.get::<usize, String>(0),
    ) {
        Err(_) => false,
        _ => true,
    }
}

pub fn unsubscribe_to_a_course(student_uui: &str, course_uuid: &str) {
    let conn = connect();

    match conn.execute(
        "DELETE FROM courses_students WHERE id_student= ?1 AND id_course= ?2;",
        &[student_uui, course_uuid],
    ) {
        Ok(deleted) => println!("{} rows were delete", deleted),
        Err(err) => println!("delete failed: {}", err),
    }
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
        "UPDATE students SET name = ?1, grade = ?2, photo = ?3, availability = ?4 WHERE guid = ?5"
    ).unwrap();

    let StudentUpdate {
        name,
        grade,
        photo,
        availability,
    } = student_update;

    stmt.execute([&name, &grade, &photo, &availability, student_uuid])
        .unwrap()
}
