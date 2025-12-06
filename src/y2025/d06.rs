//! # Trash Compactor
//!
//! We parse the input as a `Grid<u8>` and start at the very bottom right:
//!
//! - Moving left until we get the first byte that represents the column's
//!   operator
//! - Then, we move either horizontally (part 1) or vertically (part 2) to
//!   construct the operands

use crate::util::{grid::Grid, point::Point};

pub fn parse(input: &str) -> Grid<u8> {
  Grid::parse(input)
}

pub fn operate(grid: &Grid<u8>, vertical: bool) -> u64 {
  let op_row = grid.height - 1;
  let mut col_end = grid.width;
  let mut total = 0;

  while col_end > 0 {
    // Find the operator
    let mut col_start = col_end - 1;
    while grid[Point::new(col_start, op_row)] == b' ' {
      col_start -= 1;
    }

    let get_number = |num: u64, x: i32, y: i32| {
      let digit = grid[Point::new(x, y)];
      if digit.is_ascii_digit() {
        10 * num + u64::from(digit - b'0')
      } else {
        num
      }
    };

    let should_add = grid[Point::new(col_start, op_row)] == b'+';
    let answer: u64 = if vertical {
      let operands = (col_start..col_end)
        .map(|x| (0..op_row).fold(0, |num, y| get_number(num, x, y)));
      if should_add {
        operands.sum()
      } else {
        operands.product()
      }
    } else {
      let operands = (0..op_row)
        .map(|y| (col_start..col_end).fold(0, |num, x| get_number(num, x, y)));
      if should_add {
        operands.sum()
      } else {
        operands.product()
      }
    };

    total += answer;
    col_end = col_start - 1;
  }

  total
}

pub fn p1(input: &Grid<u8>) -> u64 {
  operate(input, false)
}

pub fn p2(input: &Grid<u8>) -> u64 {
  operate(input, true)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
    4_277_556
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
    3_263_827
  )]

  fn test_p2(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
