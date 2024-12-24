//! # Dive!
//!
//! We track the position of the submarine as a `Point`.
//! Given each `Command`, we simply update the position according to the
//! instructions.

use crate::util::{parse::ParseOps, point::Point};

#[derive(Clone, Copy)]
pub enum Command {
  Up(i32),
  Down(i32),
  Forward(i32),
}

pub fn parse(input: &str) -> Vec<Command> {
  input
    .trim()
    .split_ascii_whitespace()
    .array_chunks::<2>()
    .map(|[command, value]| {
      let value = value.signed();
      match command {
        "forward" => Command::Forward(value),
        "down" => Command::Down(value),
        "up" => Command::Up(value),
        _ => unreachable!(),
      }
    })
    .collect()
}

pub fn p1(input: &[Command]) -> u32 {
  let final_position =
    input
      .iter()
      .fold(Point::new(0, 0), |pos, &command| match command {
        Command::Forward(value) => pos + Point::new(value, 0),
        Command::Down(value) => pos - Point::new(0, value),
        Command::Up(value) => pos + Point::new(0, value),
      });
  (final_position.x * final_position.y).unsigned_abs()
}

pub fn p2(input: &[Command]) -> u32 {
  let (final_position, _) =
    input
      .iter()
      .fold(
        (Point::new(0, 0), 0),
        |(pos, aim), &command| match command {
          Command::Down(value) => (pos, aim - value),
          Command::Up(value) => (pos, aim + value),
          Command::Forward(value) => {
            (pos + Point::new(value, aim * value), aim)
          }
        },
      );
  (final_position.x * final_position.y).unsigned_abs()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "forward 5
down 5
forward 8
up 3
down 8
forward 2",
    150
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "forward 5
down 5
forward 8
up 3
down 8
forward 2",
    900
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
