//! # Password Philosophy
//!
//! Having parsed all the `PasswordPolicy`s, we simply iterate over them, filter
//! for valid policies, and count them.
//!
//! For part 1, we check if the count of the letter in the password is within
//! the range specified by the policy.
//!
//! For part 2, we check if only one of the
//! letters, if any, at the specified positions are the same as the letter in
//! the policy.

use crate::util::parse::ParseOps;

pub struct PasswordPolicy<'a> {
  first: usize,
  second: usize,
  letter: u8,
  password: &'a [u8],
}

impl PasswordPolicy<'_> {
  pub fn is_valid_by_frequency(&self) -> bool {
    let count = bytecount::count(self.password, self.letter);

    self.first <= count && count <= self.second
  }

  pub fn is_valid_by_position(&self) -> bool {
    let first = self.password.get(self.first - 1).copied();
    let second = self.password.get(self.second - 1).copied();

    match (first, second) {
      (Some(first), Some(second)) => {
        (first == self.letter) ^ (second == self.letter)
      }
      _ => false,
    }
  }

  pub fn from([a, b, c, d]: [&str; 4]) -> PasswordPolicy<'_> {
    let first = a.unsigned();
    let second = b.unsigned();
    let letter = c.as_bytes()[0];
    let password = d.as_bytes();
    PasswordPolicy {
      first,
      second,
      letter,
      password,
    }
  }
}

pub fn parse(input: &str) -> Vec<PasswordPolicy<'_>> {
  input
    .split(['-', ':', ' ', '\n'])
    .filter(|s| !s.is_empty())
    .array_chunks::<4>()
    .map(PasswordPolicy::from)
    .collect()
}

pub fn p1(input: &[PasswordPolicy<'_>]) -> usize {
  input
    .iter()
    .filter(|policy| policy.is_valid_by_frequency())
    .count()
}

pub fn p2(input: &[PasswordPolicy<'_>]) -> usize {
  input
    .iter()
    .filter(|policy| policy.is_valid_by_position())
    .count()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
    2
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
    1
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
