use postgres::{Client, NoTls};

pub fn connect() -> Client {
    Client::connect("host=localhost user=postgres password=postgres", NoTls).unwrap()
} 

pub fn insert_new_user(username: &str, password: &str, user_permission: Vec<String> ) -> String {

}

pub fn update_user() -> bool {}

pub fn get_user() {}