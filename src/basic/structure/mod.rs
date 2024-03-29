#![allow(dead_code)]
mod user;
mod role;

pub fn main() {
  let u1 = user::User::new("alexunder", "Alex Under");
  u1.say_hello();

  let u2 = user::User {
    username: String::from("alexunder111"),
    ..u1
  };
  u2.say_hello();

  let user::User { username, fullname, role: _} = u2;
  println!("destructure u2 into variable -> value -> username: {}, fullname: {}", username, fullname)
}
