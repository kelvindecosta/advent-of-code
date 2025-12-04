//! # Printing Department
//!
//! We parse the input as a `Grid<u8>`, storing each symbol.
//!
//! Checking for a roll of paper at each point ('@'), we count how many
//! neighboring points within the grid are also rolls.
//!
//! If a roll has fewer than 4 such neighbours, we mark it as removable.
//!
//! For part 1, we simply count the number of removable rolls after the initial
//! pass.
//!
//! For part 2, we keep track of a grid that has the neighboring roll count for
//! each point that has/had a roll, and keep removing rolls until there are none
//! that can be removed.

use crate::util::{grid::Grid, point::Point};

#[allow(clippy::cast_possible_truncation)]
pub fn parse(input: &str) -> (usize, usize) {
  let grid = Grid::parse(input.trim());
  let mut removable = Vec::new();
  let mut neighbour_counts = grid.same_size_with(u8::MAX);

  for y in 0..grid.height {
    for x in 0..grid.width {
      let point = Point::new(x, y);
      if grid[point] == b'@' {
        let count = point
          .neighbours()
          .iter()
          .filter(|&&neighbour| {
            grid.contains(neighbour) && grid[neighbour] == b'@'
          })
          .count();

        if count < 4 {
          removable.push(point);
        } else {
          neighbour_counts[point] = count as u8;
        }
      }
    }
  }

  let count1 = removable.len();
  let mut count2 = 0;

  while let Some(point) = removable.pop() {
    count2 += 1;

    for &neighbour in &point.neighbours() {
      if !neighbour_counts.contains(neighbour) {
        continue;
      }

      if neighbour_counts[neighbour] == 4 {
        removable.push(neighbour);
      }

      neighbour_counts[neighbour] -= 1;
    }
  }

  (count1, count2)
}

pub const fn p1(input: &(usize, usize)) -> usize {
  input.0
}

pub const fn p2(input: &(usize, usize)) -> usize {
  input.1
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    13
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    43
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
