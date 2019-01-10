use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum FieldState {
  Unclaimed,
  Claimed(usize),
  Contested,
}

pub fn std(input: Vec<String>) -> Option<String> {
  let pattern = Regex::new(
    r"#(?P<id>\d+) @ (?P<origin_x>\d+),(?P<origin_y>\d+): (?P<size_x>\d+)x(?P<size_y>\d+)",
  )
  .unwrap();
  let mut field = vec![vec![FieldState::Unclaimed; 1000]; 1000];
  Some(
    input
      .iter()
      .filter_map(|x| pattern.captures(x))
      .fold(0, |acc, x| {
        if let (Ok(id), Ok(origin_x), Ok(origin_y), Ok(size_x), Ok(size_y)) = (
          x["id"].parse::<usize>(),
          x["origin_x"].parse::<usize>(),
          x["origin_y"].parse::<usize>(),
          x["size_x"].parse::<usize>(),
          x["size_y"].parse::<usize>(),
        ) {
          let mut increment = 0;
          for i in origin_x..origin_x + size_x {
            for j in origin_y..origin_y + size_y {
              match field[i][j] {
                FieldState::Unclaimed => field[i][j] = FieldState::Claimed(id),
                FieldState::Claimed(_) => {
                  field[i][j] = FieldState::Contested;
                  increment += 1;
                }
                FieldState::Contested => (),
              }
            }
          }
          acc + increment
        } else {
          acc
        }
      })
      .to_string(),
  )
}

pub fn plus(input: Vec<String>) -> Option<String> {
  let pattern = Regex::new(
    r"#(?P<id>\d+) @ (?P<origin_x>\d+),(?P<origin_y>\d+): (?P<size_x>\d+)x(?P<size_y>\d+)",
  )
  .unwrap();
  let mut field = vec![vec![FieldState::Unclaimed; 1000]; 1000];
  input
    .iter()
    .filter_map(|x| pattern.captures(x))
    .fold(HashSet::new(), |mut acc, x| {
      if let (Ok(id), Ok(origin_x), Ok(origin_y), Ok(size_x), Ok(size_y)) = (
        x["id"].parse::<usize>(),
        x["origin_x"].parse::<usize>(),
        x["origin_y"].parse::<usize>(),
        x["size_x"].parse::<usize>(),
        x["size_y"].parse::<usize>(),
      ) {
        let mut uncontested = true;
        for i in origin_x..origin_x + size_x {
          for j in origin_y..origin_y + size_y {
            match field[i][j] {
              FieldState::Unclaimed => field[i][j] = FieldState::Claimed(id),
              FieldState::Claimed(claiming_id) => {
                field[i][j] = FieldState::Contested;
                uncontested = false;
                acc.remove(&claiming_id);
              }
              FieldState::Contested => uncontested = false,
            }
          }
        }
        if uncontested {
          acc.insert(id);
        }
      }
      acc
    })
    .iter()
    .nth(0)
    .map(|x| x.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn std_example_1() {
    let input = vec![
      String::from("#1 @ 1,3: 4x4"),
      String::from("#2 @ 3,1: 4x4"),
      String::from("#3 @ 5,5: 2x2"),
    ];
    let expected = Some("4".to_string());
    assert_eq!(std(input), expected);
  }

  #[test]
  fn plus_example_1() {
    let input = vec![
      String::from("#1 @ 1,3: 4x4"),
      String::from("#2 @ 3,1: 4x4"),
      String::from("#3 @ 5,5: 2x2"),
    ];
    let expected = Some("3".to_string());
    assert_eq!(plus(input), expected);
  }
}
