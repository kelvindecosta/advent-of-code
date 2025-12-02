//! # Gift Shop
//!
//! Instead of using string pattern matching, we can find the invalid numbers by
//! checking for numeric patterns.
//!
//! For part 1, we find numbers that are 11, 1010, 1111, etc., or some linear
//! combination of them. These numbers can be generated using arithmetic
//! sequences that start from the lowest number of `n` digits, and end at `10 ^
//! n - 1`. Within each range, we check for these numbers and calculate the
//! series.
//!
//! For part 2, we find numbers that are generated when there are repetitions of
//! prime length digits and filter out the cases when we count a number in both
//! cases.

use crate::util::parse::ParseOps;

/// Defines the arithmetic progression parameters for a specific repeating
/// pattern.
///
/// # Example
///
/// ```
/// let seq = SequenceParams::new(2, 1);
///
/// // This creates a sequence for 2-digit numbers made of a repeating 1-digit
/// // unit (e.g., `11`, `22`, `33` ... `99`).
///
/// assert_eq!(seq.step, 11);  // The difference between items (22 - 11 = 11)
/// assert_eq!(seq.start, 11); // The first item: 1 repeated twice
/// assert_eq!(seq.end, 99);   // The last item: 9 repeated twice
/// ```
pub struct SequenceParams {
  pub start: u64,
  pub step: u64,
  pub end: u64,
}

#[allow(clippy::cast_possible_truncation)]
impl SequenceParams {
  pub fn new(digits: u64, unit_size: u64) -> Self {
    let repeats = digits / unit_size;
    let power = 10_u64.pow(unit_size as u32);

    let step = (0..repeats).fold(0, |acc, _| acc * power + 1);

    let start = step * (power / 10);
    let end = step * (power - 1);

    Self { start, step, end }
  }

  pub fn series(&self, range_start: u64, range_end: u64) -> u64 {
    let lower = range_start.next_multiple_of(self.step).max(self.start);
    let upper = range_end.min(self.end);

    if lower <= upper {
      let count = (upper - lower) / self.step + 1;
      let last = lower + (count - 1) * self.step;

      // Sum of arithmetic series: S = n * (a1 + an) / 2
      (lower + last) * count / 2
    } else {
      0
    }
  }
}

/// Covers patterns like 11, 1212, 123123.
/// Note: This strictly covers cases where the string is split in half.
/// (It implicitly covers r=4, r=8 etc because 1111 is 11 repeated twice).
pub fn sequences_r2() -> Vec<SequenceParams> {
  let mut params = Vec::new();
  for len in 2..=10 {
    if len % 2 == 0 {
      params.push(SequenceParams::new(len, len / 2));
    }
  }
  params
}

/// Covers "More than twice" patterns that are NOT covered by r=2.
/// We look for odd primes.
pub fn sequences_primes() -> Vec<SequenceParams> {
  let mut params = Vec::new();
  // We check up to 7. r=11 requires 11 digits (usually out of bounds but safe
  // to include if needed)
  let primes = [3, 5, 7];

  for len in 2..=10 {
    for &r in &primes {
      if len % r == 0 {
        params.push(SequenceParams::new(len, len / r));
      }
    }
  }
  params
}

/// Covers numbers that appear in both `sequences_r2` and `sequences_primes`.
/// We must subtract these to avoid double counting.
/// r=6 is overlap of 2 and 3.
/// r=10 is overlap of 2 and 5.
pub fn sequences_overlaps() -> Vec<SequenceParams> {
  let mut params = Vec::new();
  let lcms = [6, 10];

  for len in 2..=10 {
    for &r in &lcms {
      if len % r == 0 {
        params.push(SequenceParams::new(len, len / r));
      }
    }
  }
  params
}

/// Sums all invalid IDs found within the ranges for the given sequences.
fn sum_invalid_ids(params_list: &[SequenceParams], ranges: &[[u64; 2]]) -> u64 {
  let mut total = 0;

  for params in params_list {
    for &[start, end] in ranges {
      total += params.series(start, end);
    }
  }

  total
}

pub fn parse(input: &str) -> (u64, u64) {
  let id_ranges: Vec<[u64; 2]> =
    input.iter_unsigned::<u64>().array_chunks::<2>().collect();

  let total_p1 = sum_invalid_ids(&sequences_r2(), &id_ranges);

  let total_p2 = total_p1 + sum_invalid_ids(&sequences_primes(), &id_ranges)
    - sum_invalid_ids(&sequences_overlaps(), &id_ranges);

  (total_p1, total_p2)
}

pub const fn p1(input: &(u64, u64)) -> u64 {
  input.0
}

pub const fn p2(input: &(u64, u64)) -> u64 {
  input.1
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
    1_227_775_554
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
    4_174_379_265
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
