//! # Sonar Sweep
//!
//! Part 1 and 2 are similar in that we are counting the number of windows
//! (size=n + 1, n=1, 3) where the first element is less than the last element.

use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<u32> {
  input.iter_unsigned().collect()
}

pub fn count_increasing_windows(input: &[u32], window_size: usize) -> usize {
  input
    .windows(window_size + 1)
    .filter(|w| w[0] < w[window_size])
    .count()
}

pub fn p1(input: &[u32]) -> usize {
  count_increasing_windows(input, 1)
}

pub fn p2(input: &[u32]) -> usize {
  count_increasing_windows(input, 3)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "199
200
208
210
200
207
240
269
260
263",
    7
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "199
200
208
210
200
207
240
269
260
263",
    5
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
