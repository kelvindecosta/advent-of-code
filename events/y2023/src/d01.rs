use std::str::FromStr;

use aho_corasick::AhoCorasick;
use eyre::Result;
use lazy_static::lazy_static;

pub static DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
pub static DIGITS_SPELLED_OUT: [&str; 9] = [
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

lazy_static! {
  pub static ref DIGITS_AC: AhoCorasick = AhoCorasick::new(DIGITS).unwrap();
  pub static ref DIGITS_SPELLED_OUT_AC: AhoCorasick =
    AhoCorasick::new([DIGITS, DIGITS_SPELLED_OUT].concat()).unwrap();
}

pub struct Calibration {
  first_digit: char,
  last_digit: char,
}

impl Calibration {
  #[must_use]
  pub fn value(&self) -> u32 {
    format!("{}{}", self.first_digit, self.last_digit)
      .parse::<u32>()
      .unwrap()
  }
}

pub struct CalibrationRecovery {
  calibration: Calibration,
}

impl FromStr for CalibrationRecovery {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    let digits = DIGITS_AC
      .find_overlapping_iter(line)
      .map(|m| DIGITS[m.pattern()])
      .collect::<Vec<_>>();

    Ok(Self {
      calibration: Calibration {
        first_digit: digits.first().unwrap().chars().next().unwrap(),
        last_digit: digits.last().unwrap().chars().next().unwrap(),
      },
    })
  }
}

pub struct BetterCalibrationRecovery {
  calibration: Calibration,
}

impl FromStr for BetterCalibrationRecovery {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    let digits = DIGITS_SPELLED_OUT_AC
      .find_overlapping_iter(line)
      // Map the spelling of the digit to the digit itself
      .map(|m| DIGITS[m.pattern().as_usize() % DIGITS.len()])
      .collect::<Vec<_>>();

    Ok(Self {
      calibration: Calibration {
        first_digit: digits.first().unwrap().chars().next().unwrap(),
        last_digit: digits.last().unwrap().chars().next().unwrap(),
      },
    })
  }
}

#[aoc(day01, part1)]
fn p1(input: &[CalibrationRecovery]) -> u32 {
  input
    .iter()
    .map(|recovery| recovery.calibration.value())
    .sum()
}

#[aoc(day01, part2)]
fn p2(input: &[BetterCalibrationRecovery]) -> u32 {
  input
    .iter()
    .map(|recovery| recovery.calibration.value())
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
