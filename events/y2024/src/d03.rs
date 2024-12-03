use std::str::FromStr;

use eyre::{Ok, Result};
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref OP_REGEX: Regex =
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
}

pub enum OperationType {
  Multiply(u16, u16),
}

pub struct Operation {
  op: OperationType,
  enabled: bool,
}

impl Operation {
  #[must_use]
  pub const fn value(&self) -> u32 {
    match &self.op {
      OperationType::Multiply(a, b) => (*a as u32) * (*b as u32),
    }
  }

  #[must_use]
  pub const fn conditional_value(&self) -> u32 {
    if self.enabled {
      self.value()
    } else {
      0
    }
  }
}

pub struct Instructions {
  operations: Vec<Operation>,
}

impl FromStr for Instructions {
  type Err = eyre::Error;

  fn from_str(line: &str) -> Result<Self> {
    let operations = OP_REGEX
      .captures_iter(line)
      .fold((vec![], true), |(mut ops, mut enabled), m| {
        let captures = m.unwrap();
        let (a, b, to_enable, to_disable) = (
          captures.get(1),
          captures.get(2),
          captures.get(3).is_some(),
          captures.get(4).is_some(),
        );

        // Conditionally enable or disable the operation
        if to_enable || to_disable {
          enabled = to_enable;
        }

        if let (Some(a), Some(b)) = (a, b) {
          ops.push(Operation {
            op: OperationType::Multiply(
              a.as_str().parse().unwrap(),
              b.as_str().parse().unwrap(),
            ),
            enabled,
          });
        }

        (ops, enabled)
      })
      .0;

    Ok(Self { operations })
  }
}

impl Instructions {
  pub fn value(&self) -> u32 {
    self.operations.iter().map(Operation::value).sum()
  }

  pub fn conditional_value(&self) -> u32 {
    self
      .operations
      .iter()
      .map(Operation::conditional_value)
      .sum()
  }
}

#[aoc(day03, part1)]
fn p1(input: &str) -> u32 {
  input.parse::<Instructions>().unwrap().value()
}

#[aoc(day03, part2)]
fn p2(input: &str) -> u32 {
  input.parse::<Instructions>().unwrap().conditional_value()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    161
  )]
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(input), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    48
  )]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(input), expected, "input: {input}");
  }
}
