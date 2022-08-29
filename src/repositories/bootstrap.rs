use rusqlite::Connection;

const DB_PATH: &str = "./src/database/guacamole.db";

pub fn init() {
    let conn = Connection::open(DB_PATH).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS students (
             guid text primary key,
             name text,
             password text not null,
             email text not null,
             grade text,
             photo text,
             availability text,
            insert_date DATETIME DEFAULT CURRENT_TIMESTAMP
         );",
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS courses (
             guid text primary key,
             prof text,
             schedule_date INTEGER,
             theme text ,
             address text,
             level text,
             comments text,
             insert_date DATETIME DEFAULT CURRENT_TIMESTAMP
         );",
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS courses_students (
             guid text primary key,
             id_student text,
             id_course text
         );",
        [],
    )
    .unwrap();
}
