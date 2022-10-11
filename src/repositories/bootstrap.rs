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
             user_permissions text,
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

    conn.execute(
            "INSERT OR IGNORE INTO students (
    guid,
    name,
    password,
    email,
    grade,
    photo,
    availability,
    user_permissions
    ) VALUES(
    '67e55044-10b1-426f-9247-bb680e5fe0c8',
    'Guillaume Girard',
    '$argon2id$v=19$m=4096,t=3,p=1$R4ofGHQ6RJg8DRpJRmOe8g$M79P1jYZJVSfAc2Wp6KzDe1Nr2ps1HNm5OeRBc+4PQ4',
    'guillaumegirardpro@gmail.com',
    'Guro ISA',
    '',
    '',
    'OP_GET_SECURED_INFO,ROLE_PROF'
    )
    ",
            [],
        )
        .unwrap();
}
