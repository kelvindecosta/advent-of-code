//! # Lobby
//!
//! For a bank of size `n` with a choice of `capacity` batteries:
//!
//! - We assume the last `capacity` batteries to be the best sequence initially.
//! - Next, we take one battery at a time from the remaining, going backwards,
//!   as our candidate to replace one of the batteries in the best sequence.
//! - If the candidate is strictly greater than the current battery in the
//!   sequence, we swap them. The displaced battery becomes the new "candidate"
//!   and we attempt to slot it into the *next* position to the right. This
//!   ensures higher values bubble to the front while preserving the relative
//!   order of the displaced chain.
//!
//! For part 1 and part 2 the `capacity` is 2 and 12 respectively.

use std::mem::replace;

pub fn parse(input: &str) -> Vec<&str> {
  input.lines().collect()
}

pub fn total_joltage<const CAPACITY: usize>(banks: &[&str]) -> u64 {
  let mut batteries = [0; CAPACITY];

  banks
    .iter()
    .map(|&bank| {
      let tail_start = bank.len() - CAPACITY;
      // Assume the last `CAPACITY` batteries to be the best sequence.
      // A benefit of doing so means we don't need to reorder them.
      batteries.copy_from_slice(&bank.as_bytes()[tail_start..]);

      // For each remaining batter (in reverse)...
      for mut candidate in bank[..tail_start].bytes().rev() {
        for battery in &mut batteries {
          // ...we try to slot the candidate into the batteries, if the current
          // battery is bigger.
          if candidate < *battery {
            break;
          }
          // If it is, we swap the candidate, and then bubble the swapped
          // battery
          candidate = replace(battery, candidate);
        }
        // In doing so, we will get rid of the candidate or one of the batteries
      }

      batteries
        .iter()
        .fold(0, |joltage, &b| 10 * joltage + u64::from(b - b'0'))
    })
    .sum()
}

pub fn p1(input: &[&str]) -> u64 {
  total_joltage::<2>(input)
}

pub fn p2(input: &[&str]) -> u64 {
  total_joltage::<12>(input)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "987654321111111
811111111111119
234234234234278
818181911112111",
    357
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "987654321111111
811111111111119
234234234234278
818181911112111",
    3_121_910_778_619
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
