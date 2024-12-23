//! # Inventory Management System
//!
//! For each box ID, we count the frequency of each letter, and whether a letter
//! was counted twice or thrice. For part 2, we track for each box ID its
//! mutated versions by replacing at each position a letter with a blank, in
//! search of the first duplicate.

use core::str;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<&[u8]> {
  input.trim().lines().map(str::as_bytes).collect()
}

pub fn p1(input: &[&[u8]]) -> u32 {
  let mut twos_count = 0;
  let mut threes_count = 0;

  for &box_id in input {
    let mut freq = [0; 26];
    let mut has_two = 0;
    let mut has_three = 0;

    for &c in box_id {
      let index = (c - b'a') as usize;
      let count = freq[index];

      match count {
        1 => has_two += 1,
        2 => {
          has_two -= 1;
          has_three += 1;
        }
        3 => has_three -= 1,
        _ => (),
      }

      freq[index] += 1;
    }

    if has_two > 0 {
      twos_count += 1;
    }

    if has_three > 0 {
      threes_count += 1;
    }
  }

  twos_count * threes_count
}

pub fn p2(input: &[&[u8]]) -> String {
  let width = input[0].len();
  let mut seen = HashSet::new();

  let mut buffer = [0; 32];

  for column in 0..width {
    for &box_id in input {
      buffer[0..width].copy_from_slice(box_id);
      buffer[column] = b'_';

      if !seen.insert(buffer) {
        return buffer
          .iter()
          .map(|&c| c as char)
          .filter(|&c| c.is_ascii_lowercase())
          .collect();
      }
    }

    seen.clear();
  }

  unreachable!();
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab",
    12
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz",
    "fgij"
  )]
  fn test_p2(#[case] input: &str, #[case] expected: String) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
