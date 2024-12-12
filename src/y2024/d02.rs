//! # Red-Nosed Reports
//!
//! Track the direction of the changes between each pair of levels, assigning:
//! - 1 for an increase,
//! - -1 for a decrease, and
//! - 0 for no change, or no valid change.
//!
//! A report is considered safe if the absolute net change is the same as the
//! number of changes.
//!
//! For part 2, we consider a report safe if eliminating one level results in a
//! safe report. This is done by discarding the change between that level and
//! its neighbors, and adding the change between those neighbors.

use crate::util::parse::ParseOps;

type Answers = (usize, usize);

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn is_safe(report: &[i32]) -> (usize, usize) {
  let net_change: i32 = report
    .windows(2)
    .map(|w| get_change_direction(w[0], w[1]))
    .sum();
  let max_change = report.len() as i32 - 1;

  if net_change.abs() == max_change {
    return (1, 1);
  }

  for index in 0..report.len() {
    let mut corrected_net_change = net_change;

    let prev_level = (index > 0).then(|| report[index - 1]);
    let next_level = (index < report.len() - 1).then(|| report[index + 1]);

    // Discard the change between the current level and its neighbors, and add
    // the change between those neighbors.
    corrected_net_change = corrected_net_change
      - prev_level.map_or(0, |prev| get_change_direction(prev, report[index]))
      - next_level.map_or(0, |next| get_change_direction(report[index], next))
      + prev_level
        .zip(next_level)
        .map_or(0, |(prev, next)| get_change_direction(prev, next));

    if corrected_net_change.abs() == (max_change - 1) {
      return (0, 1);
    }
  }

  (0, 0)
}

// Returns the change in direction between two levels.
pub fn get_change_direction(a: i32, b: i32) -> i32 {
  let diff = b - a;

  if (1..=3).contains(&diff.abs()) {
    diff.signum()
  } else {
    0
  }
}

pub fn parse(input: &str) -> Answers {
  let (strict, lenient): (Vec<usize>, Vec<usize>) = input
    .trim()
    .lines()
    .map(|line| is_safe(&line.iter_signed().collect::<Vec<_>>()))
    .unzip();

  (strict.iter().sum(), lenient.iter().sum())
}

pub const fn p1(input: &Answers) -> usize {
  input.0
}

pub const fn p2(input: &Answers) -> usize {
  input.1
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    2
  )]
  #[case("1 3 2 4 5", 0)]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    4
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
