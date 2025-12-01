//! # Secret Entrance
//!
//! For part 1, we simply check if the remainder when dividing the dial's
//! position by 100 is 0.
//!
//! For part 2, we count the number of clicks made:
//! - for right turns, we simply count the net number of turns
//! - for left turns, we invert the dial and follow the logic if it were a right
//!   turn

use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<i32> {
  let direction = input.bytes().filter(|&b| b.is_ascii_uppercase());
  let amount = input.iter_signed::<i32>();
  direction
    .zip(amount)
    .map(|(d, a)| if d == b'R' { a } else { -a })
    .collect()
}

pub fn p1(input: &[i32]) -> i32 {
  let mut dial = 50;
  let mut password = 0;

  for &rotation in input {
    dial += rotation;
    password += i32::from(dial % 100 == 0);
  }

  password
}

pub fn p2(input: &[i32]) -> i32 {
  let mut dial = 50;
  let mut password = 0;

  for &rotation in input {
    if rotation > 0 {
      password += (dial + rotation) / 100;
    } else {
      password += ((100 - dial).rem_euclid(100) - rotation) / 100;
    }
    dial = (dial + rotation).rem_euclid(100);
  }

  password
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
    3
  )]
  fn test_p1(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
    6
  )]
  fn test_p2(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
