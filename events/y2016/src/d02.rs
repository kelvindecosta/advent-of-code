use std::{cmp::min, str::FromStr};

use eyre::{bail, Result};
use lazy_static::lazy_static;

pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl TryFrom<char> for Direction {
  type Error = eyre::Error;

  fn try_from(c: char) -> Result<Self> {
    match c {
      'U' => Ok(Self::Up),
      'D' => Ok(Self::Down),
      'L' => Ok(Self::Left),
      'R' => Ok(Self::Right),
      _ => bail!("Invalid direction: {}", c),
    }
  }
}

pub struct Instruction {
  directions: Vec<Direction>,
}

impl FromStr for Instruction {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    line
      .chars()
      .map(Direction::try_from)
      .collect::<Result<Vec<_>>>()
      .map(|directions| Self { directions })
  }
}

pub struct KeyPad {
  keys: Vec<Vec<Option<char>>>,
  start: (usize, usize),
}

lazy_static! {
  pub static ref KEYPAD_P1: KeyPad = KeyPad {
    keys: vec![
      vec![Some('1'), Some('2'), Some('3')],
      vec![Some('4'), Some('5'), Some('6')],
      vec![Some('7'), Some('8'), Some('9')],
    ],
    start: (1, 1),
  };
  pub static ref KEYPAD_P2: KeyPad = KeyPad {
    keys: vec![
      vec![None, None, Some('1'), None, None],
      vec![None, Some('2'), Some('3'), Some('4'), None],
      vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
      vec![None, Some('A'), Some('B'), Some('C'), None],
      vec![None, None, Some('D'), None, None],
    ],
    start: (2, 0),
  };
}

impl KeyPad {
  pub fn follow_directions(
    &self,
    directions: &[Direction],
    start: (usize, usize),
  ) -> (usize, usize) {
    let mut position = start;
    for direction in directions {
      let new_position = match direction {
        Direction::Up => (position.0.saturating_sub(1), position.1),
        Direction::Down => {
          (min(position.0 + 1, self.keys.len() - 1), position.1)
        }
        Direction::Left => (position.0, position.1.saturating_sub(1)),
        Direction::Right => {
          (position.0, min(position.1 + 1, self.keys[0].len() - 1))
        }
      };

      let new_key = self.keys[new_position.0][new_position.1];

      if new_key.is_some() {
        position = new_position;
      }
    }

    position
  }

  pub fn press(&self, position: (usize, usize)) -> char {
    self.keys[position.0][position.1].unwrap()
  }

  pub fn code(&self, instructions: &[Instruction]) -> String {
    let mut position = self.start;
    let mut code = String::new();

    for instruction in instructions {
      position = self.follow_directions(&instruction.directions, position);
      code.push(self.press(position));
    }

    code
  }
}

#[aoc(day02, part1)]
fn p1(input: &[Instruction]) -> String {
  KEYPAD_P1.code(input)
}

#[aoc(day02, part2)]
fn p2(input: &[Instruction]) -> String {
  KEYPAD_P2.code(input)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "ULL
RRDDD
LURDL
UUUUD",
    "1985"
  )]
  fn test_p1_examples(#[case] input: &str, #[case] expected: String) {
    assert_eq!(
      p1(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }

  #[rstest]
  #[case(
    "ULL
RRDDD
LURDL
UUUUD",
    "5DB3"
  )]
  fn test_p2_examples(#[case] input: &str, #[case] expected: String) {
    assert_eq!(
      p2(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }
}
