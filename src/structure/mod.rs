#![allow(dead_code)]
mod user;

pub fn main() {
  let u1 = user::User::new("alexunder", "Alex Under");
  u1.say_hello();

  let u2 = user::User {
    username: String::from("alexunder111"),
    ..u1
  };
  u2.say_hello();
}
