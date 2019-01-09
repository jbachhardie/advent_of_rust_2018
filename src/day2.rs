use std::collections::HashMap;

fn has_repeats(input: &str, n: usize) -> bool {
  let mut seen = HashMap::new();
  for x in input.chars() {
    match seen.get_mut(&x) {
      None => {
        seen.insert(x, 1);
      }
      Some(seen_times) => {
        *seen_times += 1;
      }
    }
  }
  seen.values().any(|&x| x == n)
}

pub fn std(input: Vec<String>) -> Option<String> {
  let checksum = input.iter().filter(|&x| has_repeats(x, 2)).count()
    * input.iter().filter(|&x| has_repeats(x, 3)).count();
  Some(checksum.to_string())
}

pub fn plus(input: Vec<String>) -> Option<String> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn std_example_1() {
    let input = vec![
      String::from("abcdef"),
      String::from("bababc"),
      String::from("abbcde"),
      String::from("abcccd"),
      String::from("aabcdd"),
      String::from("abcdee"),
      String::from("ababab"),
    ];
    let expected = Some("12".to_string());
    assert_eq!(std(input), expected);
  }

  #[test]
  fn plus_example_1() {
    let input = vec![
      String::from("abcde"),
      String::from("fghij"),
      String::from("klmno"),
      String::from("pqrst"),
      String::from("fguij"),
      String::from("axcye"),
      String::from("wvxyz"),
    ];
    let expected = Some("fgij".to_string());
    assert_eq!(plus(input), expected);
  }
}
