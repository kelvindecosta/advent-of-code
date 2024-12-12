//! # Trebuchet?!
//!
//! For each line, we find the first and last byte that represents a digit,
//! construct the two-digit number and sum for all lines.
//!
//! For part 2, besides
//! checking if the byte itself is a digit, we also check if the line starts
//! (for the first digit) or ends (for the last digit) with the spelling of a
//! digit.

use crate::util::parse::ParseByte;

pub fn parse(input: &str) -> Vec<&str> {
  input.trim().lines().collect()
}

pub static DIGITS: [&[u8]; 9] = [
  b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight",
  b"nine",
];

pub fn p1(input: &[&str]) -> u32 {
  input
    .iter()
    .map(|line| {
      let first = line
        .bytes()
        .find_map(|b| b.is_ascii_digit().then_some(b.to_decimal()))
        .unwrap();

      let last = line
        .bytes()
        .rev()
        .find_map(|b| b.is_ascii_digit().then_some(b.to_decimal()))
        .unwrap();

      u32::from(first * 10 + last)
    })
    .sum()
}

pub fn p2(input: &[&str]) -> usize {
  input
    .iter()
    .map(|line| {
      let mut line = line.as_bytes();

      let first = loop {
        if let Some(value) = line[0]
          .is_ascii_digit()
          .then_some(line[0].to_decimal() as usize)
          .or_else(|| {
            DIGITS
              .iter()
              .enumerate()
              .find_map(|(i, &d)| line.starts_with(d).then_some(i + 1))
          })
        {
          break value;
        }

        line = &line[1..];
      };

      let last = loop {
        if let Some(value) = line[line.len() - 1]
          .is_ascii_digit()
          .then_some(line[line.len() - 1].to_decimal() as usize)
          .or_else(|| {
            DIGITS
              .iter()
              .enumerate()
              .find_map(|(i, &d)| line.ends_with(d).then_some(i + 1))
          })
        {
          break value;
        }

        line = &line[..line.len() - 1];
      };

      first * 10 + last
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    142
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    281
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
