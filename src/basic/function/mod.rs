#![allow(dead_code)]

mod simple;
mod lambda;

pub fn main() {
  simple::public_void();
  simple::public_ret();
  simple::public_param(1, 3);

  lambda::public_anonymous(1, 3);
}
