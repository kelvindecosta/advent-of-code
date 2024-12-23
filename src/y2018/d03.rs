//! # No Matter How You Slice It
//!
//! After parsing each `Claim`, we create a `HashMap` to store the claims for
//! each square inch of the fabric and a `HashSet` to store the ids of the
//! claims that are not overlapping.
//!
//! As we iterate over the claims, we insert for each square inch of the claim
//! the id of the claim in the `HashMap`. If the square inch already has a
//! claim, we remove the ids of the claims from the `HashSet`.

use std::collections::{HashMap, HashSet};

use crate::util::{parse::ParseOps, point::Point};

pub struct Claim {
  pub id: usize,
  pub pos: Point,
  pub dimensions: Point,
}

impl Claim {
  pub fn squares(&self) -> Vec<Point> {
    (self.pos.x..self.pos.x + self.dimensions.x)
      .flat_map(|x| {
        (self.pos.y..self.pos.y + self.dimensions.y)
          .map(move |y| Point::new(x, y))
      })
      .collect()
  }
}

type Answers = (usize, usize);

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn parse(input: &str) -> Answers {
  let claims = input
    .trim()
    .lines()
    .map(|line| {
      let parts = line.iter_unsigned().collect::<Vec<_>>();
      Claim {
        id: parts[0],
        pos: Point::new(parts[1] as i32, parts[2] as i32),
        dimensions: Point::new(parts[3] as i32, parts[4] as i32),
      }
    })
    .collect::<Vec<_>>();

  let mut fabric = HashMap::new();
  let mut non_overlapping_claims = HashSet::new();

  for claim in &claims {
    non_overlapping_claims.insert(claim.id);
    for square in claim.squares() {
      let overlaps = fabric.entry(square).or_insert(Vec::new());
      overlaps.push(claim.id);

      if overlaps.len() > 1 {
        for id in overlaps.iter() {
          non_overlapping_claims.remove(id);
        }
      }
    }
  }

  (
    fabric.values().filter(|v| v.len() > 1).count(),
    *non_overlapping_claims.iter().next().unwrap(),
  )
}

pub const fn p1(input: &Answers) -> usize {
  input.0
}

pub const fn p2(input: &Answers) -> usize {
  input.1
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
    4
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
    3
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
