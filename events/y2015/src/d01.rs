use eyre::{bail, Result};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Up = 1,
  Down = -1,
  Stay = 0,
}

impl TryFrom<char> for Direction {
  type Error = eyre::Error;

  fn try_from(value: char) -> Result<Self> {
    match value {
      '(' => Ok(Direction::Up),
      ')' => Ok(Direction::Down),
      _ => Ok(Direction::Stay),
    }
  }
}

#[aoc(day01, part1)]
fn p1(input: &[u8]) -> i32 {
  input
    .iter()
    .map(|&byte| Direction::try_from(byte as char).unwrap() as i32)
    .sum()
}

#[aoc(day01, part2)]
fn p2(input: &[u8]) -> Result<i32> {
  let mut floor = 0;
  for (i, dir) in input
    .iter()
    .map(|&byte| Direction::try_from(byte as char).unwrap() as i32)
    .enumerate()
  {
    floor += dir;
    if floor == -1 {
      return Ok((i + 1).try_into()?);
    }
  }
  bail!("Never reached the basement");
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
  fn test_p1_examples(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p1(input.as_bytes()), expected, "input: {input}");
  }

  #[rstest]
  #[case(")", 1)]
  #[case("()())", 5)]
  fn test_p2_examples(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p2(input.as_bytes()).unwrap(), expected, "input: {input}");
  }
}
