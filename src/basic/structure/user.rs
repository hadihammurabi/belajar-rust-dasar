#![allow(dead_code)]

use crate::basic::structure::role;

#[derive(Debug)]
pub struct User {
  pub username: String,
  pub fullname: String,
  pub role: role::Role,
}

impl User {

  pub fn new(username: &str, fullname: &str) -> Self {
    User {
      username: String::from(username),
      fullname: String::from(fullname),
      role: role::Role(String::from("admin")),
    }
  }

  pub fn say_hello(&self) {
    println!("Hello!! My name is {} ({}).", self.fullname, self.username);
  }

}
