use std::{collections::HashMap, str::FromStr};

use eyre::{eyre, Result};
use fancy_regex::Regex;
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
  pub fn distance_apart(&self) -> u32 {
    let [l1, l2] = self.location_ids;
    (l1 as i32 - l2 as i32).unsigned_abs()
  }
}

pub struct LocationPairList(Vec<LocationPair>);

impl LocationPairList {
  pub fn into_left_right_lists(self) -> (Vec<u32>, Vec<u32>) {
    self.0.iter().map(|pair| pair.location_ids.into()).unzip()
  }
}

impl TryFrom<Vec<LocationPair>> for LocationPairList {
  type Error = eyre::Error;

  fn try_from(value: Vec<LocationPair>) -> Result<Self> {
    Ok(Self(value))
  }
}

#[aoc(day01, part1)]
fn p1(input: &[LocationPair]) -> u32 {
  let (mut left_list, mut right_list) =
    LocationPairList::try_from(input.to_vec())
      .unwrap()
      .into_left_right_lists();

  left_list.sort_unstable();
  right_list.sort_unstable();

  let updated_pairs = left_list
    .iter()
    .zip(right_list.iter())
    .map(|(l, r)| LocationPair {
      location_ids: [*l, *r],
    })
    .collect::<Vec<_>>();

  updated_pairs.iter().map(LocationPair::distance_apart).sum()
}

#[aoc(day01, part2)]
fn p2(input: &[LocationPair]) -> u32 {
  let (left_list, right_list) = LocationPairList::try_from(input.to_vec())
    .unwrap()
    .into_left_right_lists();

  let right_occurrences =
    right_list
      .iter()
      .fold(HashMap::<u32, u32>::new(), |mut acc, &r| {
        *acc.entry(r).or_insert(0) += 1;
        acc
      });

  left_list
    .iter()
    .map(|&l| l * right_occurrences.get(&l).unwrap_or(&0))
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
