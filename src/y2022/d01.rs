//! # Calorie Counting
//!
//! After parsing each group (separated by two newlines), we sum the calorie
//! count for each group. We use a `BinaryHeap` to keep track of the top three
//! highest calorie totals.

use std::{cmp::Reverse, collections::BinaryHeap};

use crate::util::parse::ParseOps;

type Answers = (u32, u32);

pub fn parse(input: &str) -> Answers {
  let mut top_three = BinaryHeap::new();

  input
    .trim()
    .split("\n\n")
    .map(|group| group.iter_unsigned::<u32>().sum())
    .for_each(|total_calories| {
      top_three.push(Reverse(total_calories));

      // Keep the top three highest calorie totals
      if top_three.len() > 3 {
        top_three.pop();
      }
    });

  let top_three = top_three.into_sorted_vec();

  (top_three[0].0, top_three.iter().map(|r| r.0).sum())
}

pub const fn p1(input: &Answers) -> u32 {
  input.0
}

pub const fn p2(input: &Answers) -> u32 {
  input.1
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    24000
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    45000
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
