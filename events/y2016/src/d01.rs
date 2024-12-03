use std::{collections::HashSet, str::FromStr};

use eyre::{bail, Result};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy)]
pub enum Turn {
  Left = -1,
  Right = 1,
}

pub struct Movement {
  turn: Turn,
  distance: u32,
}

impl FromStr for Movement {
  type Err = eyre::Error;

  fn from_str(text: &str) -> Result<Self> {
    let turn = match text.chars().next() {
      Some('L') => Turn::Left,
      Some('R') => Turn::Right,
      _ => bail!("Invalid turn"),
    };

    let distance = text[1..].parse()?;

    Ok(Self { turn, distance })
  }
}

#[derive(Clone, Copy, Default, FromPrimitive)]
pub enum Direction {
  #[default]
  North,
  East,
  South,
  West,
}

impl Direction {
  #[must_use]
  pub fn make_turn(self, turn: Turn) -> Self {
    FromPrimitive::from_i32((self as i32 + turn as i32 + 4) % 4).unwrap()
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
  x: i32,
  y: i32,
}

impl Default for Position {
  fn default() -> Self {
    Self::origin()
  }
}

impl Position {
  #[must_use]
  pub const fn origin() -> Self {
    Self { x: 0, y: 0 }
  }

  #[must_use]
  pub const fn distance_from(self, other: Self) -> u32 {
    (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
  }

  #[must_use]
  pub fn displace(&mut self, x: i32, y: i32) -> Self {
    Self {
      x: self.x + x,
      y: self.y + y,
    }
  }
}

#[derive(Default)]
pub struct MoveState {
  position: Position,
  direction: Direction,
}

impl MoveState {
  pub fn move_forward(&mut self, distance: u32) -> Position {
    let (x, y) = match self.direction {
      Direction::North => (0, distance as i32),
      Direction::East => (distance as i32, 0),
      Direction::South => (0, -(distance as i32)),
      Direction::West => (-(distance as i32), 0),
    };
    self.position = self.position.displace(x, y);
    self.position
  }

  pub fn make_movement(&mut self, movement: &Movement) -> Position {
    self.direction = self.direction.make_turn(movement.turn);
    self.move_forward(movement.distance);
    self.position
  }
}

#[aoc(day01, part1, separator = ", ")]
fn p1(input: &[Movement]) -> u32 {
  input
    .iter()
    .fold(MoveState::default(), |mut state, movement| {
      state.make_movement(movement);
      state
    })
    .position
    .distance_from(Position::origin())
}

#[aoc(day01, part2, separator = ", ")]
fn p2(input: &[Movement]) -> Result<u32> {
  let mut visited = HashSet::new();
  let mut state = MoveState::default();

  for movement in input {
    for step in 0..movement.distance {
      let new_position = match step {
        0 => state.make_movement(&Movement {
          turn: movement.turn,
          distance: 1,
        }),
        _ => state.move_forward(1),
      };

      if !visited.insert(new_position) {
        return Ok(new_position.distance_from(Position::origin()));
      }
    }
  }

  bail!("No position visited twice")
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("R2, L3", 5)]
  #[case("R2, R2, R2", 2)]
  #[case("R5, L5, R5, R3", 12)]
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(
      p1(
        input
          .split(", ")
          .map(|part| part.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }

  #[rstest]
  #[case("R8, R4, R4, R8", 4)]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(
      p2(
        input
          .split(", ")
          .map(|part| part.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      )
      .unwrap(),
      expected,
      "input: {input}"
    );
  }
}
