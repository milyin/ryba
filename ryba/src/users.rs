use std::collections::HashMap;
use std::sync::Mutex;
type Users = HashMap<String, String>;

lazy_static! {
    static ref USERS: Mutex<Users> = Mutex::new(Users::new());
}

pub fn add_user(name: &str, password: &str) -> Result<(), String> {
    if let Ok(ref mut users) = USERS.lock() {
        if users.get(name) == None {
            users.insert(name.to_string(), password.to_string());
            Ok(())
        } else {
            Err("User exists".to_string())
        }
    } else {
        Err("Mutex locked, try again".to_string())
    }
}

pub fn check_user(name: &str, password: &str) -> Result<(), String> {
    if let Ok(ref mut users) = USERS.lock() {
        match users.get(name) {
            Some(correct_password) if password == correct_password => Ok(()),
            Some(_) => Err("wrong password".to_string()),
            None => Err("user not found".to_string()),
        }
    } else {
        Err("Mutex locked, try again".to_string())
    }
}


pub fn get_user(name: &str) -> Option<String> {
    if let Ok(ref mut users) = USERS.lock() {
        match users.get(name) {
            None => None,
            Some(password) => Some(password.clone()),
        }
    } else {
        None
    }
}