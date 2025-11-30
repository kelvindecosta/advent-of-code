//! # Not Quite Lisp
//!
//! Each character in the input represents a floor direction:
//! - `(` means to go up one floor, and is treated as +1
//! - `)` means to go down one floor, and is treated as -1
//! - all other characters are ignored, and the floor doesn't change, and are
//!   treated as 0

pub fn parse(input: &str) -> Vec<i32> {
  input
    .chars()
    .map(|c| match c {
      '(' => 1,
      ')' => -1,
      _ => 0,
    })
    .collect()
}

pub fn p1(input: &[i32]) -> i32 {
  input.iter().sum()
}

pub fn p2(input: &[i32]) -> usize {
  let mut floor = 0;

  for (i, &direction) in input.iter().enumerate() {
    floor += direction;

    // Check if we've reached the basement
    if floor == -1 {
      return i + 1;
    }
  }

  unreachable!("Should have reached basement already")
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("(())", 0)]
  #[case("()()", 0)]
  #[case("(((", 3)]
  #[case("(()(()(", 3)]
  #[case("))(((((", 3)]
  #[case("())", -1)]
  #[case("))(", -1)]
  #[case(")))", -3)]
  #[case(")())())", -3)]
  fn test_p1(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(")", 1)]
  #[case("()())", 5)]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
