use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug)]
enum GuardState {
  Asleep,
  Awake,
}

struct Log<'a> {
  date: &'a str,
  guard: &'a str,
  minutes: [GuardState; 60],
}

impl<'a> Log<'a> {
  fn new(date: &'a str, guard: &'a str) -> Log<'a> {
    Log {
      date,
      guard,
      minutes: [GuardState::Awake; 60],
    }
  }
  fn change_state(&mut self, at: usize, new_state: GuardState) {
    let minutes = &mut self.minutes;
    for i in at..minutes.len() {
      minutes[i] = new_state;
    }
  }
}

fn get_new_guard(event: &str) -> Option<&str> {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
  }
  if let Some(Some(guard_id)) = RE.captures(event).map(|x| x.get(1)) {
    Some(guard_id.as_str())
  } else {
    None
  }
}

fn process_input<'a>(input: &'a mut Vec<String>) -> Vec<Log<'a>> {
  let line_re = Regex::new(r"\[(\d{4})-(\d{2}-\d{2}) (\d{2}):(\d{2})] (.+)").unwrap();
  let mut current_guard: Option<&str> = None;
  let mut logs: Vec<Log> = Vec::new();
  input.sort();
  for line in input.iter() {
    if let Some(captures) = line_re.captures(line) {
      if let [Some(_capture), Some(_year), Some(date), Some(_hour), Some(minute_raw), Some(event)] =
        &captures
          .iter()
          .map(|x| x.map(|x| x.as_str()))
          .collect::<Vec<_>>()[..]
      {
        if let Some(guard_id) = get_new_guard(event) {
          current_guard.replace(guard_id);
        } else {
          if let Some(guard) = current_guard {
            if let Some(log) = logs.last() {
              if log.date != *date {
                logs.push(Log::new(date, guard))
              }
            } else {
              logs.push(Log::new(date, guard))
            }
          }
          if let Some(log) = logs.last_mut() {
            if let Ok(minute) = minute_raw.parse::<usize>() {
              match *event {
                "falls asleep" => log.change_state(minute, GuardState::Asleep),
                "wakes up" => log.change_state(minute, GuardState::Awake),
                &_ => (),
              }
            }
          }
        }
      }
    }
  }
  logs
}

fn find_guard_most_asleep<'a>(logs: &'a Vec<Log>) -> Option<&'a str> {
  logs
    .iter()
    .fold(HashMap::new(), |mut acc, log| {
      let entry = acc.entry(log.guard).or_insert(0);
      *entry += log
        .minutes
        .iter()
        .filter(|&&state| state == GuardState::Asleep)
        .count();
      acc
    })
    .iter()
    .max_by_key(|(_, &val)| val)
    .map(|(&key, _)| key)
}

fn find_minute_most_asleep<'a>(logs: &'a Vec<Log>, guard: &'a str) -> Option<(usize, usize)> {
  let sum_by_minute = logs
    .iter()
    .filter(|x| x.guard == guard)
    .map(|x| x.minutes)
    .fold([0; 60], |mut acc, x| {
      for i in 0..60 {
        if x[i] == GuardState::Asleep {
          acc[i] += 1;
        }
      }
      acc
    });
  sum_by_minute.iter().max().and_then(|max_value| {
    sum_by_minute
      .iter()
      .position(|x| x == max_value)
      .map(|max_index| (max_index, *max_value))
  })
}

pub fn std(mut input: Vec<String>) -> Option<String> {
  let logs = process_input(&mut input);
  find_guard_most_asleep(&logs).and_then(|sleepiest_guard| {
    find_minute_most_asleep(&logs, sleepiest_guard).and_then(|(minute_most_asleep, _)| {
      sleepiest_guard
        .parse::<usize>()
        .ok()
        .map(|guard_id| guard_id * minute_most_asleep)
        .map(|x| x.to_string())
    })
  })
}

pub fn plus(mut input: Vec<String>) -> Option<String> {
  let logs = process_input(&mut input);
  let mut guards = logs.iter().map(|x| x.guard).collect::<Vec<_>>();
  guards.sort();
  guards.dedup();
  guards
    .iter()
    .map(|guard| (guard, find_minute_most_asleep(&logs, guard).unwrap()))
    .max_by(|(_, (_, x)), (_, (_, y))| x.cmp(y))
    .and_then(|(guard, (minute, _))| guard.parse::<usize>().ok().map(|guard| guard * minute))
    .map(|x| x.to_string())
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
    let expected = Some("240".to_string());
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
    let expected = Some("4455".to_string());
    assert_eq!(plus(input), expected);
  }
}
