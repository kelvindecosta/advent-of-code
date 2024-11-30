use eyre::{bail, Result};

#[aoc(day{{day}}, part1)]
fn p1(input: &str) -> Result<String> {
  unimplemented!()
}

#[aoc(day{{day}}, part2)]
fn p2(input: &str) -> Result<String> {
  unimplemented!()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  // #[case("", Ok(""))]
  fn test_p1_examples(#[case] input: &str, #[case] expected: Result<String>) {
    assert_eq!(p1(input), expected, "input: {}", input);
  }

  #[rstest]
  // #[case("", Ok(""))]
  fn test_p2_examples(#[case] input: &str, #[case] expected: Result<String>) {
    assert_eq!(p2(input), expected, "input: {}", input);
  }
}
