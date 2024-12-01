use std::collections::HashSet;

use eyre::{bail, Result};

#[aoc(day01, part1)]
fn p1(input: &[i32]) -> i32 {
  input.iter().sum()
}

#[aoc(day01, part2)]
fn p2(input: &[i32]) -> Result<i32> {
  let mut seen = HashSet::new();
  let mut sum = 0;

  for f in input.iter().cycle() {
    if !seen.insert(sum) {
      return Ok(sum);
    }
    sum += f;
  }

  bail!("No frequency repeated");
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("+1, -2, +3, +1", 3)]
  #[case("+1, +1, +1", 3)]
  #[case("+1, +1, -2", 0)]
  #[case("-1, -2, -3", -6)]

  // #[case("", Ok(""))]
  fn test_p1_examples(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(
      p1(
        input
          .split(", ")
          .map(|word| word.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }

  #[rstest]
  #[case("+1, -2, +3, +1", 2)]
  #[case("+1, -1", 0)]
  #[case("+3, +3, +4, -2, -4", 10)]
  #[case("-6, +3, +8, +5, -6", 5)]
  #[case("+7, +7, -2, -7, -4", 14)]
  fn test_p2_examples(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(
      p2(
        input
          .split(", ")
          .map(|word| word.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      )
      .unwrap(),
      expected,
      "input: {input}"
    );
  }
}
