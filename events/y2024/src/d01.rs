use std::str::FromStr;

use eyre::{eyre, Result};
use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct LocationPair {
  location_ids: [u32; 2],
}

lazy_static! {
  pub static ref LOCATION_ID_PAIR_REGEX: Regex =
    Regex::new(r"(\d+)\s+(\d+)").unwrap();
}

impl FromStr for LocationPair {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    LOCATION_ID_PAIR_REGEX
      .captures(line)
      .unwrap()
      .unwrap()
      .iter()
      .skip(1)
      .map(
        |dim| {
          dim
            .unwrap()
            .as_str()
            .parse::<u32>()
            .map_err(|e| eyre!("Failed to parse location ID pair: {e}"))
        }, // Convert ParseIntError to eyre::Error
      )
      .collect::<Result<Vec<_>>>()
      .map(|location_ids| Self {
        location_ids: location_ids.try_into().unwrap(),
      })
  }
}

impl LocationPair {
  #[must_use]
  pub const fn distance_apart(self) -> u32 {
    let [l1, l2] = self.location_ids;
    (l1 as i32 - l2 as i32).unsigned_abs()
  }
}

#[aoc(day01, part1)]
fn p1(input: &[LocationPair]) -> u32 {
  let (mut left_list, mut right_list): (Vec<_>, Vec<_>) =
    input.iter().map(|pair| pair.location_ids.into()).unzip();

  left_list.sort_unstable();
  right_list.sort_unstable();

  left_list
    .iter()
    .zip(right_list.iter())
    .map(|(&l, &r)| LocationPair {
      location_ids: [l, r],
    })
    .map(LocationPair::distance_apart)
    .sum()
}

#[aoc(day01, part2)]
fn p2(input: &[LocationPair]) -> u32 {
  let (left_list, right_list): (Vec<_>, Vec<_>) =
    input.iter().map(|pair| pair.location_ids.into()).unzip();

  let right_occurrences = right_list.into_iter().counts();

  left_list
    .into_iter()
    .map(|l| l * (right_occurrences.get(&l).copied().unwrap_or(0) as u32))
    .sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "3   4
4   3
2   5
1   3
3   9
3   3",
    11
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
    "3   4
4   3
2   5
1   3
3   9
3   3",
    31
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
