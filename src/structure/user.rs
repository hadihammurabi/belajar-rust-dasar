#![allow(dead_code)]
#[derive(Debug)]

pub struct User {
  pub username: String,
  pub fullname: String,
}

pub fn new_user(username: &str, fullname: &str) -> User {
  User {
    username: String::from(username),
    fullname: String::from(fullname),
  }
}

impl User {
  pub fn say_hello(&self) {
    println!("Hello!! My name is {} ({}).", self.fullname, self.username);
  }
}
