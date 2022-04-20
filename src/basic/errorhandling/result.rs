#![allow(dead_code)]


pub fn basic() {
  println!("errorhandling::basic");

  let mut result: Result<&str, &str> = Ok("operation success");
  println!("is_ok: {}", result.is_ok());
  println!("is_err: {}", result.is_err());
  println!("value: {}", match result.ok() {
    Some(text) => text,
    None => "",
  });

  println!("");

  result = Err("fail while doing this");
  println!("is_ok: {}", result.is_ok());
  println!("is_err: {}", result.is_err());
  println!("value: {}", match result.err() {
    Some(text) => text,
    None => "",
  });

}

pub fn unwrap() {
  println!("");
  println!("errorhandling::unwrap");

  let mut result: Result<&str, &str> = Ok("operation success");
  println!("is_ok: {}", result.is_ok());
  println!("is_err: {}", result.is_err());
  println!("value: {}", result.unwrap());

  println!("");

  result = Err("fail while doing this");
  println!("is_ok: {}", result.is_ok());
  println!("is_err: {}", result.is_err());
  println!("value: {}", result.unwrap_err());
}
