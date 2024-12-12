//! # Squares With Three Sides
//!
//! Parse the input as a list of integers, chunk into groups of 3, and filter
//! out invalid triangles.
//!
//! For part 2, we need to parse the triangles
//! vertically, so we need to split the input into 3 separate iterators.

use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<u32> {
  input.iter_unsigned().collect()
}

pub const fn is_triangle(sides: &[u32; 3]) -> bool {
  sides[0] + sides[1] > sides[2]
    && sides[0] + sides[2] > sides[1]
    && sides[1] + sides[2] > sides[0]
}

pub fn count_valid_triangles(iter: impl Iterator<Item = u32>) -> usize {
  iter
    .array_chunks::<3>()
    .filter(|&[a, b, c]| is_triangle(&[a, b, c]))
    .count()
}

pub fn p1(input: &[u32]) -> usize {
  count_valid_triangles(input.iter().copied())
}

pub fn p2(input: &[u32]) -> usize {
  count_valid_triangles(input.iter().copied().step_by(3))
    + count_valid_triangles(input.iter().copied().skip(1).step_by(3))
    + count_valid_triangles(input.iter().copied().skip(2).step_by(3))
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("5 10 25", 0)]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603",
    6
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
