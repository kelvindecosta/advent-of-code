use std::str::FromStr;

use eyre::{bail, Result};

pub struct Thing {}

impl FromStr for Thing {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    unimplemented!()
  }
}

#[aoc(day{{day}}, part1)]
fn p1(input: &[Thing]) -> Result<String> {
  unimplemented!()
}

#[aoc(day{{day}}, part2)]
fn p2(input: &[Thing]) -> Result<String> {
  unimplemented!()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  // #[case("", Ok(""))]
  fn test_p1_examples(#[case] input: &str, #[case] expected: Result<String>) {
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
  fn test_p2_examples(#[case] input: &str, #[case] expected: Result<String>) {
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
