#![allow(dead_code)]

#[derive(Debug)]
pub struct User {
  pub username: String,
  pub fullname: String,
}

impl User {

  pub fn new(username: &str, fullname: &str) -> Self {
    User {
      username: String::from(username),
      fullname: String::from(fullname),
    }
  }

  pub fn say_hello(&self) {
    println!("Hello!! My name is {} ({}).", self.fullname, self.username);
  }

}
