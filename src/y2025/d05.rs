//! # Cafeteria
//!
//! We merge all the ranges (after having sorted them based on their start ID)
//! to remove any overlaps. For part 1, we apply a binary search to quickly look
//! up whether the ID of the ingredients in our list is within one of the
//! ranges. For part 2, we count the number of IDs in each range, after having
//! merged them.

use crate::util::parse::ParseOps;

pub fn merge(intervals: &[[u64; 2]]) -> Vec<[u64; 2]> {
  if intervals.is_empty() {
    return vec![];
  }
  let mut sorted_intervals = intervals.to_vec();
  sorted_intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

  let mut merged: Vec<[u64; 2]> = Vec::new();
  merged.push(sorted_intervals[0]);

  for interval in sorted_intervals.iter().skip(1) {
    let last = merged.last_mut().unwrap();

    if interval[0] <= last[1] {
      last[1] = last[1].max(interval[1]);
    } else {
      merged.push(*interval);
    }
  }

  merged
}

pub fn parse(input: &str) -> (Vec<[u64; 2]>, Vec<u64>) {
  let (ranges, ingredients) = input.trim().split_once("\n\n").unwrap();

  (
    merge(
      &ranges
        .iter_unsigned()
        .array_chunks::<2>()
        .collect::<Vec<_>>(),
    ),
    ingredients.iter_unsigned().collect(),
  )
}

pub fn p1(input: &(Vec<[u64; 2]>, Vec<u64>)) -> usize {
  input
    .1
    .iter()
    .filter(|&&id| {
      input
        .0
        .binary_search_by(|&range| {
          if id < range[0] {
            std::cmp::Ordering::Greater
          } else if id > range[1] {
            std::cmp::Ordering::Less
          } else {
            std::cmp::Ordering::Equal
          }
        })
        .is_ok()
    })
    .count()
}

pub fn p2(input: &(Vec<[u64; 2]>, Vec<u64>)) -> u64 {
  input
    .0
    .iter()
    .fold(0, |acc, &range| acc + (range[1] - range[0]) + 1)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    3
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    14
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
