use std::str::FromStr;

use aho_corasick::AhoCorasick;
use eyre::{eyre, Result};
use lazy_static::lazy_static;

static DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
static DIGITS_SPELLED_OUT: [&str; 9] = [
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

lazy_static! {
  static ref DIGITS_AC: AhoCorasick = AhoCorasick::new(DIGITS).unwrap();
  static ref DIGITS_SPELLED_OUT_AC: AhoCorasick =
    AhoCorasick::new([DIGITS, DIGITS_SPELLED_OUT].concat()).unwrap();
}

#[derive(Debug)]
struct Calibration {
  value: u32,
}

impl FromStr for Calibration {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    let digits = DIGITS_AC
      .find_overlapping_iter(line)
      .map(|m| DIGITS[m.pattern()])
      .collect::<Vec<_>>();

    [*digits.first().unwrap(), *digits.last().unwrap()]
      .join("")
      .parse::<u32>()
      .map(|value| Self { value })
      .map_err(|e| eyre!("Failed to parse calibration: {e}"))
  }
}

#[derive(Debug)]
struct BetterCalibration {
  value: u32,
}

impl FromStr for BetterCalibration {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    let digits = DIGITS_SPELLED_OUT_AC
      .find_overlapping_iter(line)
      .map(|m| DIGITS[m.pattern().as_usize() % DIGITS.len()])
      .collect::<Vec<_>>();

    [*digits.first().unwrap(), *digits.last().unwrap()]
      .join("")
      .parse::<u32>()
      .map(|value| Self { value })
      .map_err(|e| eyre!("Failed to parse calibration: {e}"))
  }
}

#[aoc(day01, part1)]
fn p1(input: &[Calibration]) -> u32 {
  input.iter().map(|calibration| calibration.value).sum()
}

#[aoc(day01, part2)]
fn p2(input: &[BetterCalibration]) -> u32 {
  input.iter().map(|calibration| calibration.value).sum()
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
  #[case("coneightfivedfkqrfjcckghzsrtrc9sevenone1", 11)]
  #[case("oneight", 18)]
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
