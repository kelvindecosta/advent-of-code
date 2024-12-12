//! # Spiral Memory
//!
//! ## Part 1
//!
//! When the spiral is written down, the number at the bottom right at each
//! level, is always of the form `(2n + 1)^2` where `n` is the level/ ring.
//! The distance to the center is the sum of:
//! - the distance from teh center to the ring, `n`
//! - the distance from the number ot the center of the corresponding side.
//!
//! ## Part 2
//!
//! Store the values as Points in a `HashMap`, with the value as the sum of the
//! values of the neighbours.
use std::collections::HashMap;

use crate::util::{
  parse::ParseOps,
  point::{Point, RIGHT},
};

pub fn parse(input: &str) -> u32 {
  input.unsigned()
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn p1(input: &u32) -> u32 {
  let ring = f64::from(*input).sqrt().ceil() as u32 / 2;
  if ring == 0 {
    0
  } else {
    ring + (((*input - 1) % (2 * ring)).abs_diff(ring))
  }
}

pub fn p2(input: &u32) -> u32 {
  let mut pos = Point::new(0, 0);
  let mut spiral = HashMap::new();
  spiral.insert(pos, 1);

  let mut dir = RIGHT;

  while spiral[&pos] <= *input {
    pos += dir;

    spiral.insert(
      pos,
      pos
        .neighbours()
        .iter()
        .map(|p| spiral.get(p).unwrap_or(&0))
        .sum(),
    );

    if pos.x.abs() == (pos.y.abs() + i32::from(dir == RIGHT)) {
      dir = dir.counter_clockwise();
    }
  }

  spiral[&pos]
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("1", 0)]
  #[case("12", 3)]
  #[case("23", 2)]
  #[case("1024", 31)]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("3", 4)]
  #[case("4", 5)]
  #[case("5", 10)]
  #[case("10", 11)]
  #[case("11", 23)]
  #[case("26", 54)]
  #[case("59", 122)]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
