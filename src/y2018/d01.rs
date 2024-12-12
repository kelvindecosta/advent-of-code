//! # Chronal Calibration
//!
//! Parse the input as a list of signed integers.
//! For part 2, use a `HashSet` to keep track of the cumulative frequencies
//! we've seen.

use std::collections::HashSet;

use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<i32> {
  input.iter_signed().collect()
}

pub fn p1(input: &[i32]) -> i32 {
  input.iter().sum()
}

pub fn p2(input: &[i32]) -> i32 {
  let mut seen = HashSet::new();
  let mut sum = 0;

  for f in input.iter().cycle() {
    if !seen.insert(sum) {
      return sum;
    }
    sum += f;
  }

  unreachable!("No frequency repeated");
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("+1, -2, +3, +1", 3)]
  #[case("+1, +1, +1", 3)]
  #[case("+1, +1, -2", 0)]
  #[case("-1, -2, -3", -6)]
  fn test_p1(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("+1, -2, +3, +1", 2)]
  #[case("+1, -1", 0)]
  #[case("+3, +3, +4, -2, -4", 10)]
  #[case("-6, +3, +8, +5, -6", 5)]
  #[case("+7, +7, -2, -7, -4", 14)]
  fn test_p2(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
