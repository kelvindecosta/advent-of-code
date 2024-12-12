//! # Historian Hysteria
//!
//! Parse the two lists of integers by chunking them into pairs and then
//! unzipping.

use itertools::Itertools;

use crate::util::parse::ParseOps;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
  input
    .iter_unsigned::<u32>()
    .array_chunks::<2>()
    .map(Into::into)
    .unzip()
}

pub fn p1(input: &Input) -> u32 {
  let (mut left, mut right) = input.clone();
  left.sort_unstable();
  right.sort_unstable();

  left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

pub fn p2(input: &Input) -> usize {
  let (left, right) = input.clone();
  let right_occurrences = right.iter().counts();
  left
    .iter()
    .map(|l| (*l as usize) * (right_occurrences.get(l).copied().unwrap_or(0)))
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
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
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
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
