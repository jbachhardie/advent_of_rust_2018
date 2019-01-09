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

fn no_differring_chars(a: &str, b: &str) -> String {
  let mut result = String::new();
  for i in 0..a.len() {
    if let (Some(character_a), Some(character_b)) = (a.chars().nth(i), b.chars().nth(i)) {
      if character_a == character_b {
        result.push(character_a);
      }
    }
  }
  result
}

pub fn plus(input: Vec<String>) -> Option<String> {
  let mut ids = input.clone();
  loop {
    match ids.pop() {
      Some(candidate) => match ids
        .iter()
        .map(|x| no_differring_chars(&candidate, x))
        .find(|x| x.len() == candidate.len() - 1)
      {
        Some(result) => break Some(result),
        None => continue,
      },
      None => break None,
    }
  }
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
