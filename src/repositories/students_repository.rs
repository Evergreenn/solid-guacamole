use rusqlite::{Connection};
use uuid::Uuid;

const DB_PATH: &str = dotenv!("DATABASE_PATH");

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub guid: String,
    pub email: String,
    pub password: String,
}

pub fn connect() -> Connection {
    Connection::open(DB_PATH).unwrap()
}

pub fn insert_new_user(email: &str, password: &str, _user_permission: Vec<String>) -> String {
    let conn = connect();

    let uuid = Uuid::new_v4();

    match conn.execute(
        "INSERT INTO students (guid, email, password) values (?1, ?2, ?3);",
        &[&uuid.to_string().to_owned(), email, &password],
    ) {
        Ok(inserted) => println!("{} rows were inserted", inserted),
        Err(err) => println!("insert failed: {}", err),
    }

    let guid = conn
        .query_row(
            "SELECT guid FROM students WHERE guid = ?1;",
            [uuid.to_string()],
            |row| row.get(0),
        )
        .unwrap();

    guid
}

pub fn is_user_exists(email: &str) -> bool {
    let conn = connect();

    match conn.query_row(
        "SELECT email FROM students WHERE email = ?1;",
        &[email],
        |row| row.get::<usize, String>(0),
    ) {
        Err(_) => false,
        _ => true,
    }
}



// pub fn update_user() -> bool {}

pub fn get_user(username: &str) -> Vec<User> {

    let conn = connect();


    let mut stmt = conn.prepare(
        "SELECT guid, email, password FROM students WHERE email = ?1",
    ).unwrap();

    let users = stmt.query_map(&[username], |row| 
        Ok(User{
            guid: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
        })
    ).unwrap();

    let mut to_return : Vec<User> = vec![];

    for user in users {
        to_return.push(user.unwrap());
    }

    to_return
}
