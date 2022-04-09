#![allow(dead_code)]

mod clone;
mod copy;
mod reference;

pub fn main() {
  copy::copy();
  clone::clone();
  reference::reference();
}
