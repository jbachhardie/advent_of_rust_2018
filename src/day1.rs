use std::collections::HashSet;

pub fn std(input: Vec<String>) -> Option<String> {
  Some(
    input
      .iter()
      .map(|x| x.parse::<i32>().unwrap())
      .fold(0, |acc, x| acc + x)
      .to_string(),
  )
}

pub fn plus(input: Vec<String>) -> Option<String> {
  let mut seen = HashSet::new();
  let mut current = 0;
  seen.insert(current);
  input
    .iter()
    .map(|x| x.parse::<i32>().unwrap())
    .cycle()
    .find_map(|x| {
      current = current + x;
      if seen.contains(&current) {
        Some(current.to_string())
      } else {
        seen.insert(current);
        None
      }
    })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn std_example_1() {
    let input = vec![
      String::from("+1"),
      String::from("-2"),
      String::from("+3"),
      String::from("+1"),
    ];
    let expected = Some("3".to_string());
    assert_eq!(std(input), expected);
  }

  #[test]
  fn std_example_2() {
    let input = vec![String::from("+1"), String::from("+1"), String::from("+1")];
    let expected = Some("3".to_string());
    assert_eq!(std(input), expected);
  }

  #[test]
  fn std_example_3() {
    let input = vec![String::from("+1"), String::from("+1"), String::from("-2")];
    let expected = Some("0".to_string());
    assert_eq!(std(input), expected);
  }

  #[test]
  fn std_example_4() {
    let input = vec![String::from("-1"), String::from("-2"), String::from("-3")];
    let expected = Some("-6".to_string());
    assert_eq!(std(input), expected);
  }

  #[test]
  fn plus_example_1() {
    let input = vec![
      String::from("+1"),
      String::from("-2"),
      String::from("+3"),
      String::from("+1"),
    ];
    let expected = Some("2".to_string());
    assert_eq!(plus(input), expected);
  }

  #[test]
  fn plus_example_2() {
    let input = vec![String::from("+1"), String::from("-1")];
    let expected = Some("0".to_string());
    assert_eq!(plus(input), expected);
  }

  #[test]
  fn plus_example_3() {
    let input = vec![
      String::from("+3"),
      String::from("+3"),
      String::from("+4"),
      String::from("-2"),
      String::from("-4"),
    ];
    let expected = Some("10".to_string());
    assert_eq!(plus(input), expected);
  }

  #[test]
  fn plus_example_4() {
    let input = vec![
      String::from("-6"),
      String::from("+3"),
      String::from("+8"),
      String::from("+5"),
      String::from("-6"),
    ];
    let expected = Some("5".to_string());
    assert_eq!(plus(input), expected);
  }

  #[test]
  fn plus_example_5() {
    let input = vec![
      String::from("+7"),
      String::from("+7"),
      String::from("-2"),
      String::from("-7"),
      String::from("-4"),
    ];
    let expected = Some("14".to_string());
    assert_eq!(plus(input), expected);
  }
}
