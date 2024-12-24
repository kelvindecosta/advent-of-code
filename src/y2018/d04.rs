//! # Repose Record
//!
//! After sorting the records chronologically (in this case, lexicographically),
//! we track each guard's sleeping patterns using a `HashMap` where the key is
//! the guard's ID and the value is an array of 60 elements representing the
//! minutes in an hour.

use std::collections::HashMap;

use crate::util::parse::ParseOps;

type Input = HashMap<usize, [u32; 60]>;

pub fn parse(input: &str) -> Input {
  let mut records = input.trim().lines().collect::<Vec<_>>();
  records.sort_unstable();

  let mut guards = HashMap::new();
  let mut current_guard = None;
  let mut sleep_start = None;

  for record in records {
    match record.len() {
      // Guard falls asleep
      31 => {
        sleep_start = Some((&record[15..17]).unsigned::<usize>());
      }
      // Guard wakes up
      27 => {
        let sleep_end = (&record[15..17]).unsigned();

        let guard_sleep_tracker = guards
          .entry(current_guard.unwrap())
          .or_insert_with(|| [0; 60]);
        (sleep_start.unwrap()..sleep_end)
          .for_each(|i| guard_sleep_tracker[i] += 1);
      }
      // Guard begins shift
      _ => {
        current_guard = Some((&record[26..record.len() - 13]).unsigned());
      }
    }
  }

  guards
}

fn choose(
  input: &Input,
  strategy: impl Fn(&(&usize, &[u32; 60])) -> u32,
) -> usize {
  // Find the guard using the strategy
  let (id, sleep_tracker) = input.iter().max_by_key(strategy).unwrap();

  // Find the minute when the guard is sleeping the most frequently
  let (minute, _) = sleep_tracker
    .iter()
    .enumerate()
    .max_by_key(|(_, &freq)| freq)
    .unwrap();

  id * minute
}

pub fn p1(input: &Input) -> usize {
  choose(input, |(_, sleep_tracker)| sleep_tracker.iter().sum())
}

pub fn p2(input: &Input) -> usize {
  choose(input, |(_, sleep_tracker)| {
    *sleep_tracker.iter().max().unwrap()
  })
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
    240
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
    4455
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
