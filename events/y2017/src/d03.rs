use std::collections::HashMap;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Direction {
  Right,
  Up,
  Left,
  Down,
}

impl Direction {
  #[must_use]
  pub fn next(&self) -> Self {
    // Going counter-clockwise, so add 1 to the current direction
    FromPrimitive::from_u32((*self as u32 + 1) % 4).unwrap()
  }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Default)]
pub struct Position {
  x: i32,
  y: i32,
}

impl Position {
  #[must_use]
  pub fn next(&self, direction: Direction) -> (Self, Direction) {
    let (x_diff, y_diff) = match direction {
      Direction::Up => (0, 1),
      Direction::Down => (0, -1),
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
    };

    let next_position = Self {
      x: self.x + x_diff,
      y: self.y + y_diff,
    };

    let next_direction = if next_position.x.abs()
      == next_position.y.abs()
        + match direction {
          Direction::Right => 1,
          _ => 0,
        } {
      direction.next()
    } else {
      direction
    };

    (next_position, next_direction)
  }

  #[must_use]
  pub fn neighbours(&self) -> Vec<Self> {
    (-1..=1)
      .flat_map(|x| (-1..=1).map(move |y| (x, y)))
      // Skip the origin
      .filter(|&(x, y)| x != 0 || y != 0)
      .map(|(x, y)| Self {
        x: self.x + x,
        y: self.y + y,
      })
      .collect()
  }
}

#[aoc(day03, part1)]
fn p1(input: &str) -> u32 {
  let num: u32 = input.trim().parse().unwrap();
  // The last number of the nth ring will be the square of the nth odd number
  let ring = (num as f64).sqrt().ceil() as u32 / 2;
  if ring == 0 {
    0
  } else {
    // The net distance from the center is the sum of:
    // - the distance from the center to the ring
    // - the distance from the number to the center of the corresponding side of
    //   the ring
    ring + (((num - 1) % (2 * ring)) as i32 - ring as i32).unsigned_abs()
  }
}

#[aoc(day03, part2)]
fn p2(input: &str) -> u32 {
  let num: u32 = input.trim().parse().unwrap();
  let mut pos = Position::default();
  let mut spiral = HashMap::new();
  spiral.insert(pos.clone(), 1);

  let mut dir = Direction::Right;

  while spiral[&pos] <= num {
    (pos, dir) = pos.next(dir);

    spiral.insert(
      pos.clone(),
      pos
        .neighbours()
        .iter()
        .map(|p| spiral.get(p).unwrap_or(&0))
        .sum(),
    );
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
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(input), expected, "input: {input}");
  }

  #[test]
  fn test_neighbours() {
    let pos = Position::default();
    let neighbours = pos.neighbours();
    assert_eq!(neighbours.len(), 8);
    assert_eq!(
      neighbours,
      vec![
        Position { x: -1, y: -1 },
        Position { x: -1, y: 0 },
        Position { x: -1, y: 1 },
        Position { x: 0, y: -1 },
        Position { x: 0, y: 1 },
        Position { x: 1, y: -1 },
        Position { x: 1, y: 0 },
        Position { x: 1, y: 1 },
      ]
    );
  }

  #[rstest]
  #[case("3", 4)]
  #[case("4", 5)]
  #[case("5", 10)]
  #[case("10", 11)]
  #[case("11", 23)]
  #[case("26", 54)]
  #[case("59", 122)]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(input), expected, "input: {input}");
  }
}
