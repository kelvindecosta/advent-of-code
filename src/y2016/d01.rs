//! # No Time for a Taxicab
//!
//! Parse all movements and simple follow them to reach the final position.
//!
//! For part 2, we track all the positions along the way and find the distance
//! to the first position that we visit twice.

use std::collections::HashSet;

use crate::util::{
  parse::ParseOps,
  point::{Point, ORIGIN, UP},
};
pub struct Movement {
  turn: char,
  distance: i32,
}

impl Movement {
  pub fn make_turn(&self, direction: Point) -> Point {
    match self.turn {
      'L' => direction.counter_clockwise(),
      'R' => direction.clockwise(),
      _ => unreachable!(),
    }
  }
}

pub fn parse(input: &str) -> Vec<Movement> {
  let turns = input.chars().filter(char::is_ascii_alphabetic);
  let distances = input.iter_signed();

  turns
    .zip(distances)
    .map(|(turn, distance)| Movement { turn, distance })
    .collect()
}

pub fn p1(input: &[Movement]) -> i32 {
  let mut position = ORIGIN;
  let mut direction = UP;

  for movement in input {
    direction = movement.make_turn(direction);
    position += direction * movement.distance;
  }

  position.manhattan(ORIGIN)
}

pub fn p2(input: &[Movement]) -> i32 {
  let mut position = ORIGIN;
  let mut direction = UP;

  let mut visited = HashSet::new();

  for movement in input {
    direction = movement.make_turn(direction);

    for _ in 0..movement.distance {
      position += direction;

      if !visited.insert(position) {
        return position.manhattan(ORIGIN);
      }
    }
  }

  unreachable!()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("R2, L3", 5)]
  #[case("R2, R2, R2", 2)]
  #[case("R5, L5, R5, R3", 12)]
  fn test_p1(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("R8, R4, R4, R8", 4)]
  fn test_p2(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
