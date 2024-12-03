use std::collections::HashSet;

use eyre::{bail, Result};

#[derive(Clone, Copy)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl TryFrom<char> for Direction {
  type Error = eyre::Error;

  fn try_from(value: char) -> Result<Self> {
    match value {
      '^' => Ok(Self::Up),
      'v' => Ok(Self::Down),
      '<' => Ok(Self::Left),
      '>' => Ok(Self::Right),
      _ => bail!("Invalid direction: {value}"),
    }
  }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Default)]
pub struct Position {
  x: i32,
  y: i32,
}

impl Position {
  #[must_use]
  pub fn deliver(&mut self, direction: Direction) -> Self {
    let (x_diff, y_diff) = match direction {
      Direction::Up => (0, 1),
      Direction::Down => (0, -1),
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
    };

    Self {
      x: self.x + x_diff,
      y: self.y + y_diff,
    }
  }
}

#[aoc(day03, part1)]
fn p1(input: &[u8]) -> u32 {
  let mut visited = HashSet::new();
  visited.insert(Position::default());

  input
    .iter()
    .map(|&byte| Direction::try_from(byte as char).unwrap())
    .fold(Position::default(), |mut position, direction| {
      position = position.deliver(direction);
      visited.insert(position);
      position
    });

  visited.len() as u32
}

#[aoc(day03, part2)]
fn p2(input: &[u8]) -> u32 {
  let mut visited = HashSet::new();
  visited.insert(Position::default());

  // Split the input into two halves and process them separately
  for index in 0..2 {
    input
      .iter()
      .skip(index)
      .step_by(2)
      .map(|&byte| Direction::try_from(byte as char).unwrap())
      .fold(Position::default(), |mut position, direction| {
        position = position.deliver(direction);
        visited.insert(position);
        position
      });
  }

  visited.len() as u32
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(">", 2)]
  #[case("^>v<", 4)]
  #[case("^v^v^v^v^v", 2)]
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(input.as_bytes()), expected, "input: {input}");
  }

  #[rstest]
  #[case("^v", 3)]
  #[case("^>v<", 3)]
  #[case("^v^v^v^v^v", 11)]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(input.as_bytes()), expected, "input: {input}");
  }
}
