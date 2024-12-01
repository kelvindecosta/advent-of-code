use std::collections::HashMap;

use eyre::{bail, Result};

#[aoc(day01, part1)]
fn p1(input: &[i32]) -> Result<i32> {
  let mut complements = HashMap::new();

  for expense in input {
    if let Some(&complement) = complements.get(expense) {
      return Ok(expense * complement);
    }

    let complement = 2020 - expense;
    complements.insert(complement, expense);
  }

  bail!("No solution found");
}

#[aoc(day01, part2)]
fn p2(input: &[i32]) -> Result<i32> {
  let mut seen = Vec::new();
  let mut complements = HashMap::new();

  for exp1 in input {
    if complements.contains_key(exp1) {
      let (exp2, exp3) = complements[exp1];
      return Ok(exp1 * exp2 * exp3);
    }

    for exp2 in &seen {
      complements.insert(2020 - (exp1 + exp2), (*exp1, *exp2));
    }

    seen.push(*exp1);
  }

  bail!("No solution found");
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "1721
979
366
299
675
1456",
    514_579
  )]
  fn test_p1_examples(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(
      p1(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      )
      .unwrap(),
      expected,
      "input: {input}"
    );
  }

  #[rstest]
  #[case(
    "1721
979
366
299
675
1456",
    241_861_950
  )]
  fn test_p2_examples(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(
      p2(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      )
      .unwrap(),
      expected,
      "input: {input}"
    );
  }
}
