//! # Lobby
//!
//! For a bank of size `n` with a choice of `capacity` batteries:
//!
//! - The first digit is found by taking the maximum of the first `n - capacity`
//!   jotlages.
//! - The next digit in the sequence is found by taking the maximum of the first
//!   `n - capacity - 1` joltages, except for those that are before the previous
//!   maximum.
//! - We repeat until we've got `capacity` digits
//!
//! For part 1 and part 2 the `capacity` is 2 and 12 respectively.

pub fn parse(input: &str) -> Vec<&[u8]> {
  input.lines().map(str::as_bytes).collect()
}

pub fn total_joltage(banks: &[&[u8]], capacity: usize) -> u64 {
  banks
    .iter()
    .map(|&bank| {
      let mut current_max = 0;
      let mut current_start = 0;

      (0..capacity).rev().fold(0, |joltage, place: usize| {
        let window_end = bank.len() - place;

        (current_max, current_start) = (current_start..window_end).fold(
          (0, 0),
          |(window_max, window_start), i| {
            if bank[i] > window_max {
              (bank[i], i + 1)
            } else {
              (window_max, window_start)
            }
          },
        );

        10 * joltage + u64::from(current_max - b'0')
      })
    })
    .sum()
}

pub fn p1(input: &[&[u8]]) -> u64 {
  total_joltage(input, 2)
}

pub fn p2(input: &[&[u8]]) -> u64 {
  total_joltage(input, 12)
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
