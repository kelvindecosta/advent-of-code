//! # Corruption Checksum
//!
//! When parsing the spreadsheet, the values in each row are sorted in ascending
//! order. This eases the calculation of both the min-max checksum and the
//! evenly divisible pair checksum, as we make the least amount of comparisons.

use itertools::Itertools;

use crate::util::parse::ParseOps;

pub struct SpreadSheet {
  rows: Vec<Vec<u32>>,
}

pub fn parse(input: &str) -> SpreadSheet {
  SpreadSheet {
    rows: input
      .trim()
      .lines()
      .map(|line| {
        let mut values: Vec<_> = line.iter_unsigned().collect();
        values.sort_unstable();
        values
      })
      .collect(),
  }
}

impl SpreadSheet {
  pub fn get_min_max_checksum(&self) -> u32 {
    self
      .rows
      .iter()
      .map(|values| values[values.len() - 1] - values[0])
      .sum()
  }

  pub fn get_evenly_divisible_pair_checksum(&self) -> u32 {
    self
      .rows
      .iter()
      .map(|values| {
        values
          .iter()
          .combinations(2)
          .find_map(|pair| {
            let (&v, &w) = (pair[0], pair[1]);
            // The quotient of the two values if one is divisible by the other.
            // Dividing the larger value by the smaller value
            if v % w == 0 {
              Some(v / w)
            } else if w % v == 0 {
              Some(w / v)
            } else {
              None
            }
          })
          .unwrap()
      })
      .sum()
  }
}

pub fn p1(input: &SpreadSheet) -> u32 {
  input.get_min_max_checksum()
}

pub fn p2(input: &SpreadSheet) -> u32 {
  input.get_evenly_divisible_pair_checksum()
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
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "5 9 2 8
9 4 7 3
3 8 6 5",
    9
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
