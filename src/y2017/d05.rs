//! # A Maze of Twisty Trampolines, All Alike
//!
//! For part 1 the instruction is simply incremented, while for part 2 it is
//! decremented if the value is greater than 2.

use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<i32> {
  input.iter_signed().collect()
}

#[allow(clippy::cast_sign_loss)]
fn count_jumps(input: &[i32], offset_fn: impl Fn(i32) -> i32) -> usize {
  let mut steps = 0;
  let mut position = 0;
  let mut instructions = input.to_vec();

  while position < input.len() {
    let next = position.wrapping_add(instructions[position] as usize);
    instructions[position] += offset_fn(instructions[position]);
    steps += 1;
    position = next;
  }

  steps
}

pub fn p1(input: &[i32]) -> usize {
  count_jumps(input, |_| 1)
}

pub fn p2(input: &[i32]) -> usize {
  count_jumps(input, |v| if v > 2 { -1 } else { 1 })
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "0
3
0
1
-3",
    5
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "0
3
0
1
-3",
    10
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
