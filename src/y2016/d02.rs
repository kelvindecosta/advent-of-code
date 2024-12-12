//! # Bathroom Security
//!
//! Store the keypad as a grid and follow the directions to find the code.
//! Always check if the new position is within the grid and if the letter at the
//! new position is not a space.

use crate::util::{grid::Grid, point::Point};

type Input = Vec<Vec<Point>>;

pub fn parse(input: &str) -> Input {
  input
    .trim()
    .lines()
    .map(|line| line.bytes().map(Point::from).collect())
    .collect()
}

pub struct Keypad {
  grid: Grid<u8>,
  start: Point,
}

impl Keypad {
  pub fn follow_directions(
    &self,
    directions: &[Point],
    start: &Point,
  ) -> Point {
    let mut position = *start;
    for direction in directions {
      let new_position = position + *direction;
      if new_position.x >= 0
        && new_position.x < self.grid.width
        && new_position.y >= 0
        && new_position.y < self.grid.height
        && self.grid[new_position] != b' '
      {
        position = new_position;
      }
    }
    position
  }

  pub fn press(&self, position: &Point) -> char {
    self.grid[*position] as char
  }

  pub fn code(&self, instructions: &[Vec<Point>]) -> String {
    let mut position = self.start;
    let mut code = String::new();

    for line in instructions {
      position = self.follow_directions(line, &position);
      code.push(self.press(&position));
    }

    code
  }
}

pub fn p1(input: &Input) -> String {
  Keypad {
    grid: Grid::parse("123\n456\n789"),
    start: Point::new(1, 1),
  }
  .code(input)
}

pub fn p2(input: &Input) -> String {
  Keypad {
    grid: Grid::parse("  1  \n 234 \n56789\n ABC \n  D  "),
    start: Point::new(0, 2),
  }
  .code(input)
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
  fn test_p1(#[case] input: &str, #[case] expected: String) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "ULL
RRDDD
LURDL
UUUUD",
    "5DB3"
  )]
  fn test_p2(#[case] input: &str, #[case] expected: String) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
