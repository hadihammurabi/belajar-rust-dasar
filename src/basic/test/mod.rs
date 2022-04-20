#[cfg(test)]
mod tests {
  #[test]
  fn plus_1_and_3_is_4() {
    let actual = 1 + 3;
    let expected = 4;
    assert_eq!(actual, expected);
  }

  #[test]
  fn minus_3_and_2_is_1() {
    let actual = 3 - 2;
    let expected = 1;
    assert_eq!(actual, expected);
  }

}
