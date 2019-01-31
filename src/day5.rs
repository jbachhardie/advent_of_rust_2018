use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Unit {
  class: char,
  polarity: bool,
}

impl Unit {
  fn new(c: char) -> Self {
    Unit {
      class: c.to_ascii_uppercase(),
      polarity: c.is_ascii_uppercase(),
    }
  }
  fn reacts_with(&self, candidate: &Unit) -> bool {
    self.class == candidate.class && self.polarity != candidate.polarity
  }
}

#[derive(Clone)]
struct Polymer(Vec<Unit>);

impl Polymer {
  fn react_once(&mut self) {
    let mut i = 0;
    while i + 1 < self.len() {
      if self.0[i].reacts_with(&self.0[i + 1]) {
        self.0.remove(i);
        self.0.remove(i);
      } else {
        i += 1;
      }
    }
  }

  fn react(&mut self) {
    let mut prev_len = self.len();
    loop {
      self.react_once();
      if self.len() == prev_len {
        break;
      } else {
        prev_len = self.len();
      }
    }
  }

  fn remove_unit_of_class(&mut self, &class: &char) {
    let mut i = 0;
    while i < self.len() {
      if self.0[i].class == class {
        self.0.remove(i);
      } else {
        i += 1;
      }
    }
  }

  fn len(&self) -> usize {
    self.0.len()
  }
}

impl std::str::FromStr for Polymer {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Polymer(
      s.chars().map(|x| Unit::new(x)).collect::<Vec<Unit>>(),
    ))
  }
}

impl IntoIterator for Polymer {
  type Item = Unit;
  type IntoIter = ::std::vec::IntoIter<Unit>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

pub fn std(input: Vec<String>) -> Option<String> {
  let mut polymer = input[0].parse::<Polymer>().unwrap();
  polymer.react();
  Some(polymer.len().to_string())
}

pub fn plus(mut input: Vec<String>) -> Option<String> {
  let mut possible_lengths: HashMap<char, usize> = HashMap::new();
  let polymer = input[0].parse::<Polymer>().unwrap();
  for unit in polymer.clone() {
    if !possible_lengths.contains_key(&unit.class) {
      let mut subpolymer = polymer.clone();
      subpolymer.remove_unit_of_class(&unit.class);
      subpolymer.react();
      possible_lengths.insert(unit.class, subpolymer.len());
    }
  }
  possible_lengths
    .iter()
    .map(|x| x.1)
    .min()
    .map(|x| x.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn std_example_1() {
    let input = vec!["dabAcCaCBAcCcaDA".to_string()];
    let expected = Some("10".to_string());
    assert_eq!(std(input), expected);
  }

  #[test]
  fn plus_example_1() {
    let input = vec!["dabAcCaCBAcCcaDA".to_string()];
    let expected = Some("4".to_string());
    assert_eq!(plus(input), expected);
  }
}
