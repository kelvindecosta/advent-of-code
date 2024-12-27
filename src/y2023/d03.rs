//! # Gear Ratios
//!
//! We parse the input as a `Grid<u8>`, storing each digit/symbol.
//! The part numbers are stored in a `Vec<u32>`, with the index of each number
//! in this vector marked in the `seen` grid, at the location of each of the
//! number's digits.
//!
//! ## Part 1
//!
//! We iterate over each cell in the grid, in search of each non-digit.
//! For each of their neighbours, we add the part number to the result, if it
//! hasn't been added yet.
//!
//! ## Part 2
//!
//! For each cell with a `*`, we iterate over its neighbours, left-to-right,
//! top-to-bottom. We multiply all the part numbers we find, and add the result
//! to the final sum if we find exactly two distinct parts.
//!
//! > This solution was heavily inspired by [`maneatingape`'s solution](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day03.rs).

use crate::util::{grid::Grid, parse::ParseByte, point::Point};

pub struct Schematic {
  grid: Grid<u8>,
  seen: Grid<usize>,
  parts: Vec<u32>,
}

pub fn parse(input: &str) -> Schematic {
  let grid = Grid::parse(input);
  // In order to tell if we've already seen a number before, store its index at
  // the location of every digit, using zero to indicate no value.
  let mut seen: Grid<usize> = grid.same_size_with(0);

  // Store each unique part number.
  let mut parts = vec![0];
  let mut number = 0;

  for y in 0..grid.height {
    for x in 0..grid.width {
      let p = Point::new(x, y);
      let b = grid[p];

      if b.is_ascii_digit() {
        // Parse contiguous groups of digits.
        seen[p] = parts.len();
        number = 10 * number + u32::from(b.to_decimal());
      } else if number > 0 {
        // If not a digit, store the number and reset.
        parts.push(number);
        number = 0;
      }
    }

    // Finish the number at the end of the line.
    if number > 0 {
      parts.push(number);
      number = 0;
    }
  }

  Schematic { grid, seen, parts }
}

pub fn p1(input: &Schematic) -> u32 {
  let Schematic { grid, seen, parts } = input;
  let mut parts = parts.clone();
  let mut result = 0;

  for y in 0..grid.height {
    for x in 0..grid.width {
      let p = Point::new(x, y);
      let b = grid[p];

      if !b.is_ascii_digit() && b != b'.' {
        for neighbour in p.neighbours() {
          let part_index = seen[neighbour];
          if part_index != 0 {
            result += parts[part_index];
            // Prevent double counting.
            parts[part_index] = 0;
          }
        }
      }
    }
  }

  result
}

pub fn p2(input: &Schematic) -> u32 {
  let Schematic { grid, seen, parts } = input;
  let mut result = 0;

  for y in 0..grid.height {
    for x in 0..grid.width {
      let p = Point::new(x, y);

      if grid[p] == b'*' {
        let mut previous_part_index = 0;
        let mut distinct_parts = 0;
        let mut subtotal = 1;

        for neighbour in p.neighbours() {
          let part_index = seen[neighbour];
          if part_index != 0 && part_index != previous_part_index {
            previous_part_index = part_index;
            distinct_parts += 1;
            subtotal *= parts[part_index];
          }
        }

        if distinct_parts == 2 {
          result += subtotal;
        }
      }
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    4361
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    467_835
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
