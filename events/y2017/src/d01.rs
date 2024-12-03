use std::str::FromStr;

use eyre::{eyre, Result};

pub struct Digit {
  value: u8,
}

impl TryFrom<char> for Digit {
  type Error = eyre::Error;

  fn try_from(value: char) -> Result<Self> {
    value
      .to_digit(10)
      .map(|value| Self { value: value as u8 })
      .ok_or_else(|| eyre!("Invalid digit: {value}"))
  }
}

pub struct Captcha {
  digits: Vec<Digit>,
}

impl Captcha {
  #[must_use]
  pub fn solve(&self, offset: usize) -> u32 {
    self
      .digits
      .iter()
      .enumerate()
      .filter(|(i, digit)| {
        digit.value == self.digits[(i + offset) % self.digits.len()].value
      })
      .map(|(_, digit)| digit.value as u32)
      .sum::<u32>()
  }
}

impl FromStr for Captcha {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    line
      .chars()
      .map(Digit::try_from)
      .collect::<Result<Vec<_>>>()
      .map(|digits| Self { digits })
  }
}

#[aoc(day01, part1)]
fn p1(input: &[Captcha]) -> u32 {
  input[0].solve(1)
}

#[aoc(day01, part2)]
fn p2(input: &[Captcha]) -> u32 {
  input[0].solve(input[0].digits.len() / 2)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("1122", 3)]
  #[case("1111", 4)]
  #[case("1234", 0)]
  #[case("91212129", 9)]
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(
      p1(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }

  #[rstest]
  #[case("1212", 6)]
  #[case("1221", 0)]
  #[case("123425", 4)]
  #[case("123123", 12)]
  #[case("12131415", 4)]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(
      p2(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }
}
