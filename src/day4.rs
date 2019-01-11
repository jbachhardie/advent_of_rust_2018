use regex::Regex;

#[derive(Copy, Clone)]
enum GuardState {
  Asleep,
  Awake,
}

pub fn std(mut input: Vec<String>) -> Option<String> {
  let line_re = Regex::new(r"\[(\d{4})-(\d{2}-\d{2}) (\d{2}):(\d{2})] (.+)").unwrap();
  let event_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
  let mut current_guard: Option<&str> = None;
  let mut current_day: Option<&mut (&str, Option<&str>, [GuardState; 60])> = None;
  let mut logs: Vec<(&str, Option<&str>, [GuardState; 60])> = Vec::new();
  input.sort();
  for line in input.iter() {
    if let Some(captures) = line_re.captures(line) {
      if let [Some(year), Some(date), Some(hour), Some(minute_raw), Some(event)] = &captures
        .iter()
        .map(|x| x.map(|x| x.as_str()))
        .collect::<Vec<_>>()[..]
      {
        if let Some(Some(guard_id)) = event_re.captures(event).map(|x| x.get(1)) {
          current_guard = Some(guard_id.as_str());
        } else {
          match current_day {
            Some((current_date, _, _)) if current_date == date => (),
            _ => {
              logs.push((date, current_guard, [GuardState::Awake; 60]));
              let last_index = logs.len() - 1;
              current_day = Some(&mut logs[last_index]);
            }
          }
          if let Ok(minute) = minute_raw.parse::<usize>() {
            if *event == "falls asleep" {
              current_day.map(|(_, _, mut log)| {
                for i in minute..log.len() {
                  log[i] = GuardState::Asleep;
                }
              });
            } else if *event == "wakes up" {
              current_day.map(|(_, _, mut log)| {
                for i in minute..log.len() {
                  log[i] = GuardState::Awake;
                }
              });
            }
          }
        }
      }
    }
  }
  None
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
      "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
      "[1518-11-01 00:05] falls asleep".to_string(),
      "[1518-11-01 00:25] wakes up".to_string(),
      "[1518-11-01 00:30] falls asleep".to_string(),
      "[1518-11-01 00:55] wakes up".to_string(),
      "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
      "[1518-11-02 00:40] falls asleep".to_string(),
      "[1518-11-02 00:50] wakes up".to_string(),
      "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
      "[1518-11-03 00:24] falls asleep".to_string(),
      "[1518-11-03 00:29] wakes up".to_string(),
      "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
      "[1518-11-04 00:36] falls asleep".to_string(),
      "[1518-11-04 00:46] wakes up".to_string(),
      "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
      "[1518-11-05 00:45] falls asleep".to_string(),
      "[1518-11-05 00:55] wakes up".to_string(),
    ];
    let expected = Some("4".to_string());
    assert_eq!(std(input), expected);
  }

  #[test]
  fn plus_example_1() {
    let input = vec![
      "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
      "[1518-11-01 00:05] falls asleep".to_string(),
      "[1518-11-01 00:25] wakes up".to_string(),
      "[1518-11-01 00:30] falls asleep".to_string(),
      "[1518-11-01 00:55] wakes up".to_string(),
      "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
      "[1518-11-02 00:40] falls asleep".to_string(),
      "[1518-11-02 00:50] wakes up".to_string(),
      "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
      "[1518-11-03 00:24] falls asleep".to_string(),
      "[1518-11-03 00:29] wakes up".to_string(),
      "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
      "[1518-11-04 00:36] falls asleep".to_string(),
      "[1518-11-04 00:46] wakes up".to_string(),
      "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
      "[1518-11-05 00:45] falls asleep".to_string(),
      "[1518-11-05 00:55] wakes up".to_string(),
    ];
    let expected = Some("3".to_string());
    assert_eq!(plus(input), expected);
  }
}
