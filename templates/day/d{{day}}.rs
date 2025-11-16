//! # {{title}}
//!
//! Use this docstring to explain the problem and how it is solved.

pub fn parse(input: &str) -> Input {
  unimplemented!()
}

pub fn p1(input: &Input) -> OutputP1 {
  unimplemented!()
}

pub fn p2(input: &Input) -> OutputP2 {
  unimplemented!()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("example", 0)]
  fn test_p1(#[case] input: &str, #[case] expected: OutputP1) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("example", 0)]
  fn test_p2(#[case] input: &str, #[case] expected: OutputP2) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
