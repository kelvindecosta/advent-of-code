use std::str::FromStr;

use eyre::{eyre, Result};
use itertools::Itertools;

pub struct Triplet {
  side_lengths: [u32; 3],
}

impl Triplet {
  #[must_use]
  pub const fn is_valid(&self) -> bool {
    let [a, b, c] = self.side_lengths;
    a + b > c && a + c > b && b + c > a
  }
}

impl FromStr for Triplet {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    line
      .split_whitespace()
      .map(|side| {
        side
          .parse::<u32>()
          .map_err(|e| eyre!("Failed to parse side length: {e}"))
      })
      .collect::<Result<Vec<_>>>()
      .map(|side_lengths| Self {
        side_lengths: side_lengths.try_into().unwrap(),
      })
  }
}

#[aoc(day03, part1)]
fn p1(input: &[Triplet]) -> u32 {
  input.iter().filter(|triplet| triplet.is_valid()).count() as u32
}

#[aoc(day03, part2)]
fn p2(input: &[Triplet]) -> u32 {
  input
    .iter()
    .map(|triplet| triplet.side_lengths)
    .chunks(3)
    .into_iter()
    .flat_map(|chunk| {
      let chunk = chunk.collect::<Vec<_>>();

      (0..3).map(move |i| {
        let side_lengths = chunk.iter().map(|x| x[i]);
        Triplet {
          side_lengths: side_lengths.collect::<Vec<_>>().try_into().unwrap(),
        }
      })
    })
    .filter(Triplet::is_valid)
    .count() as u32
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("5 10 25", 0)]
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
  // #[case("", Ok(""))]
  #[case(
    "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603",
    6
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
