//! # Camp Cleanup
//!
//! For part 1, we check if either one of the pair completely overlaps the
//! other.
//!
//! For part 2, we check if neither pair has any overlap and invert this condition using [DeMorgan's Law](https://en.wikipedia.org/wiki/De_Morgan%27s_laws).
//! This only works under the assumption that the pairs are ordered.

use crate::util::parse::ParseOps;

type Pair = [u32; 4];

pub fn parse(input: &str) -> Vec<Pair> {
  input.iter_unsigned().array_chunks::<4>().collect()
}

pub fn p1(input: &[Pair]) -> usize {
  input
    .iter()
    .filter(|&&[a, b, c, d]| (a >= c && b <= d) || (c >= a && d <= b))
    .count()
}

pub fn p2(input: &[Pair]) -> usize {
  input
    .iter()
    .filter(|&&[a, b, c, d]| a <= d && c <= b)
    .count()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    2
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    4
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
