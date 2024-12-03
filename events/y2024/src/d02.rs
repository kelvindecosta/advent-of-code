use std::str::FromStr;

use eyre::{eyre, Result};

pub struct Report {
  levels: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChangeDirection {
  Increase,
  Decrease,
}

impl FromStr for Report {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    line
      .split(' ')
      .map(|part| {
        part
          .parse()
          .map_err(|e| eyre!("Failed to parse int: {}", e))
      })
      .collect::<Result<Vec<_>>>()
      .map(|levels| Self { levels })
  }
}

#[must_use]
pub fn find_bad_level(levels: &[i32]) -> Option<usize> {
  let mut prev_change_direction = None;

  for (index, level) in levels.iter().skip(1).enumerate() {
    let change = level - levels[index];
    let change_abs = change.abs();

    // Change must be between 1 and 3
    if !(1..=3).contains(&change_abs) {
      return Some(index);
    }

    let curr_change_direction = match change {
      _ if change > 0 => Some(ChangeDirection::Increase),
      _ if change < 0 => Some(ChangeDirection::Decrease),
      _ => None,
    };

    // Change direction must not change
    if curr_change_direction.is_some_and(|curr| {
      prev_change_direction.is_some_and(|prev| prev != curr)
    }) {
      return Some(index);
    }

    prev_change_direction = curr_change_direction;
  }

  None
}

impl Report {
  #[must_use]
  pub fn is_safe(&self) -> bool {
    find_bad_level(&self.levels).is_none()
  }

  #[must_use]
  pub fn is_safe_within_tolerance(&self) -> bool {
    let bad_level = find_bad_level(&self.levels);

    if bad_level.is_none() {
      return true;
    }

    let bad_level = bad_level.unwrap() as isize;

    // Try removing either the bad level or the level before or after it
    [bad_level - 1, bad_level, bad_level + 1]
      .iter()
      .filter(|&index| (0..self.levels.len() as isize).contains(index))
      .find_map(|&index_to_remove| {
        let levels = self
          .levels
          .iter()
          .enumerate()
          .filter_map(|(index, &level)| {
            if (index as isize) == index_to_remove {
              None
            } else {
              Some(level)
            }
          })
          .collect::<Vec<_>>();

        if find_bad_level(&levels).is_none() {
          Some(true)
        } else {
          None
        }
      })
      .unwrap_or(false)
  }
}

#[aoc(day02, part1)]
fn p1(input: &[Report]) -> u32 {
  input.iter().filter(|&report| report.is_safe()).count() as u32
}

#[aoc(day02, part2)]
fn p2(input: &[Report]) -> u32 {
  input
    .iter()
    .filter(|&report| report.is_safe_within_tolerance())
    .count() as u32
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    2
  )]
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(
      p1(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }

  #[rstest]
  #[case(
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    4
  )]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(
      p2(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }
}
