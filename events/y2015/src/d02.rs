use std::str::FromStr;

use eyre::{eyre, Result};
use fancy_regex::Regex;
use lazy_static::lazy_static;

pub struct Gift {
  dimensions: [u32; 3],
}

lazy_static! {
  pub static ref GIFT_DIMENSIONS_REGEX: Regex =
    Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
}

impl FromStr for Gift {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    GIFT_DIMENSIONS_REGEX
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
            .map_err(|e| eyre!("Failed to parse dimension: {e}"))
        }, // Convert ParseIntError to eyre::Error
      )
      .collect::<Result<Vec<_>>>()
      .map(|dimensions| Self {
        dimensions: dimensions.try_into().unwrap(),
      })
  }
}

impl Gift {
  #[must_use]
  pub fn wrapping_paper_area(&self) -> u32 {
    let face_areas = self
      .dimensions
      .iter()
      .enumerate()
      .map(|(i, &dim)| dim * self.dimensions[(i + 1) % 3])
      .collect::<Vec<u32>>();

    2 * face_areas.iter().sum::<u32>() + face_areas.iter().min().unwrap()
  }

  #[must_use]
  pub fn ribbon_length(&self) -> u32 {
    let face_perimeters = self
      .dimensions
      .iter()
      .enumerate()
      .map(|(i, &dim)| 2 * (dim + self.dimensions[(i + 1) % 3]))
      .collect::<Vec<u32>>();

    face_perimeters.iter().min().unwrap()
      + self.dimensions.iter().product::<u32>()
  }
}

#[aoc(day02, part1)]
fn p1(input: &[Gift]) -> u32 {
  input.iter().map(Gift::wrapping_paper_area).sum()
}

#[aoc(day02, part2)]
fn p2(input: &[Gift]) -> u32 {
  input.iter().map(Gift::ribbon_length).sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("2x3x4", 58)]
  #[case("1x1x10", 43)]
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
      "input: {input}",
    );
  }

  #[rstest]
  #[case("2x3x4", 34)]
  #[case("1x1x10", 14)]
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
      "input: {input}",
    );
  }
}
