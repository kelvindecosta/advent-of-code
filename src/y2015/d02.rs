//! # I Was Told There Would Be No Math
//!
//! All integers in the input are parsed and then chunked into arrays of 3.
//! Each chunk represents the dimensions of one gift.
//!
//! Dimensions are sorted in ascending order to facilitate calculations on the
//! smallest side.

use crate::util::parse::ParseOps;

pub struct Gift {
  dimensions: [u32; 3],
}

impl Gift {
  pub const fn wrapping_paper_area(&self) -> u32 {
    let [l, w, h] = self.dimensions;
    2 * (w * h + h * l) + 3 * l * w
  }

  pub const fn ribbon_length(&self) -> u32 {
    let [l, w, h] = self.dimensions;
    2 * (l + w) + l * w * h
  }
}

pub fn parse(input: &str) -> Vec<Gift> {
  input
    .iter_unsigned()
    .array_chunks::<3>()
    .map(|chunk| {
      let mut dimensions = chunk;
      dimensions.sort_unstable();
      Gift { dimensions }
    })
    .collect()
}

pub fn p1(input: &[Gift]) -> u32 {
  input.iter().map(Gift::wrapping_paper_area).sum()
}

pub fn p2(input: &[Gift]) -> u32 {
  input.iter().map(Gift::ribbon_length).sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("2x3x4", 58)]
  #[case("1x1x10", 43)]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("2x3x4", 34)]
  #[case("1x1x10", 14)]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
