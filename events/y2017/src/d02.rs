use std::str::FromStr;

use eyre::Result;
use itertools::{Itertools, MinMaxResult::MinMax};

pub struct SpreadSheetRow {
  values: Vec<u32>,
}

impl FromStr for SpreadSheetRow {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    let values = line
      .split_whitespace()
      .map(|s| s.parse().map_err(Into::into))
      .collect::<Result<Vec<_>>>()?;
    Ok(Self { values })
  }
}

impl SpreadSheetRow {
  pub fn get_min_max_checksum(&self) -> u32 {
    match self.values.iter().minmax() {
      MinMax(v1, v2) => v2 - v1,
      _ => unreachable!(),
    }
  }

  pub fn get_evenly_divisible_pair_checksum(&self) -> u32 {
    self
      .values
      .iter()
      .combinations(2)
      .find_map(|pair| {
        let (&v, &w) = (pair[0], pair[1]);
        if v % w == 0 {
          Some(v / w)
        } else if w % v == 0 {
          Some(w / v)
        } else {
          None
        }
      })
      .unwrap()
  }
}

#[aoc(day02, part1)]
fn p1(input: &[SpreadSheetRow]) -> u32 {
  input.iter().map(SpreadSheetRow::get_min_max_checksum).sum()
}

#[aoc(day02, part2)]
fn p2(input: &[SpreadSheetRow]) -> u32 {
  input
    .iter()
    .map(SpreadSheetRow::get_evenly_divisible_pair_checksum)
    .sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "5 1 9 5
7 5 3
2 4 6 8",
    18
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
    "5 9 2 8
9 4 7 3
3 8 6 5",
    9
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
