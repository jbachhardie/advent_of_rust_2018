pub fn std<I>(input: I) -> std::result::Result<i32, std::num::ParseIntError>
where
  I: IntoIterator<Item = String>,
{
  input
    .into_iter()
    .map(|x| x.parse::<i32>())
    .fold(Ok(0), |acc, x| Ok(acc? + x?))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn std_example_1() {
    let input = vec![
      String::from("1"),
      String::from("-2"),
      String::from("3"),
      String::from("1"),
    ];
    let expected = Ok(3);
    assert_eq!(std(input), expected);
  }

  #[test]
  fn std_example_2() {
    let input = vec![String::from("1"), String::from("1"), String::from("1")];
    let expected = Ok(3);
    assert_eq!(std(input), expected);
  }

  #[test]
  fn std_example_3() {
    let input = vec![String::from("1"), String::from("1"), String::from("-2")];
    let expected = Ok(0);
    assert_eq!(std(input), expected);
  }

  #[test]
  fn std_example_4() {
    let input = vec![String::from("-1"), String::from("-2"), String::from("-3")];
    let expected = Ok(-6);
    assert_eq!(std(input), expected);
  }
}
