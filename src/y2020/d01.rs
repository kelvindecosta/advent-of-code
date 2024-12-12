//! # Report Repair
//!
//! For part 1, we track the complement (sum - value) of each expense in the
//! input, via a `HashMap`. If we find an expense whose complement is already in
//! the map, we return the product of the two.
//!
//! For part 2, we do the same, but track the complement of the sum of each pair
//! of expenses we've seen so far.

use std::collections::HashMap;

use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<u32> {
  input.iter_unsigned().collect()
}

pub fn p1(input: &[u32]) -> u32 {
  let mut complements = HashMap::new();

  for exp in input {
    if let Some(&exp1) = complements.get(exp) {
      return exp1 * exp;
    }

    if *exp > 2020 {
      continue;
    }

    let exp2 = 2020 - exp;
    complements.insert(exp2, exp);
  }

  unreachable!("No solution found");
}

pub fn p2(input: &[u32]) -> u32 {
  let mut seen = Vec::new();
  let mut complements = HashMap::new();

  for exp in input {
    if complements.contains_key(exp) {
      let (exp1, exp2) = complements[exp];
      return exp1 * exp2 * exp;
    }

    for exp1 in &seen {
      let pair_sum = exp1 + exp;
      if pair_sum > 2020 {
        continue;
      }

      complements.insert(2020 - (exp1 + exp), (*exp1, *exp));
    }

    seen.push(*exp);
  }

  unreachable!("No solution found");
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
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
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
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
