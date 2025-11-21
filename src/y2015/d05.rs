//! # Doesn't He Have Intern-Elves For This?
//!
//! ## Part 1
//!
//! Since each string consists of only lowercase ASCII characters, we can map
//! each of them into a bitmask that fits into an `i32`. We use this to quickly
//! check for vowels.
//!
//! To check for the invalid sequences ("ab', "cd", "pq" & "xy"), we use a
//! similar mask. The previous character is shifted to the left, logically
//! `AND`ed with the current character and the mask.
//!
//! ## Part 2
//!
//! We create an array (of size 27 ^ 2 to account for all pairs) based map
//! (functionally equivalent to a `HashMap` with the key being the pair of
//! characters), to track the first position at which we've seen a particular
//! pair of characters. When we've come across a pair we've seen before, we
//! check if the stored value in the map is larger than the current position by
//! 1, indicating that we've found non-overlapping pairs.
//!
//! > This solution was heavily inspired by [`maneatingape`'s solution](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2015/day05.rs).
//! > Having previously implemented a solution for this problem using regular
//! > expressions, I was curious as to how one could go about solving it without
//! > them.

pub fn parse(input: &str) -> Vec<&[u8]> {
  input.trim().lines().map(str::as_bytes).collect()
}

//                         u_______o______i____e____a
const VOWELS_MASK: u32 = 0b1_0000_0100_0001_0001_0001;
//                                  y_x_______q_p______________dcba
const FORBIDDEN_PAIRS_MASK: u32 = 0b1_0000_0001_0000_0000_0000_1010;

pub fn criteria1(line: &&&[u8]) -> bool {
  let mut num_vowels = 0;
  let mut num_pairs = 0;
  let mut previous: Option<u32> = None;

  for &c in line.iter() {
    let current = 1 << (c - b'a');

    if FORBIDDEN_PAIRS_MASK & current & (previous.unwrap_or_default() << 1) != 0
    {
      return false;
    }
    if VOWELS_MASK & current != 0 {
      num_vowels += 1;
    }
    if previous.unwrap_or_default() == current {
      num_pairs += 1;
    }

    previous = Some(current);
  }
  num_vowels >= 3 && num_pairs >= 1
}

pub fn p1(input: &[&[u8]]) -> usize {
  input.iter().filter(criteria1).count()
}

pub fn criteria2(line: &&&[u8]) -> bool {
  let mut pairs = [0; 729]; // 27 ^ 2

  let mut first: Option<usize> = None;
  let mut second: Option<usize> = None;

  let mut has_two_pair = false;
  let mut has_split_pair = false;

  for (offset, c) in line.iter().enumerate() {
    let third = (c - b'a' + 1) as usize;
    let index = 27 * second.unwrap_or_default() + third;

    let position = 1000 + offset;
    let delta = position - pairs[index];

    if delta > offset {
      // Store the position of the first occurrence of the pair
      pairs[index] = position;
    } else if delta > 1 {
      // If we've seen the pair before, and the positions differ by more than 1,
      // these are non-overlapping pairs.
      has_two_pair = true;
    }

    if first.unwrap_or_default() == third {
      has_split_pair = true;
    }

    first = second.or(None);
    second = Some(third);
  }

  has_two_pair && has_split_pair
}

pub fn p2(input: &[&[u8]]) -> usize {
  input.iter().filter(criteria2).count()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("ugknbfddgicrmopn", true)]
  #[case("aaa", true)]
  #[case("jchzalrnumimnmhp", false)]
  #[case("haegwjzuvuyypxyu", false)]
  #[case("dvszwmarrgswjxmb", false)]
  fn test_criteria1(#[case] input: &str, #[case] expected: bool) {
    assert_eq!(
      criteria1(&&str::as_bytes(input)),
      expected,
      "input: {input}"
    );
  }

  #[rstest]
  #[case("qjhvhtzxzqqjkmpb", true)]
  #[case("xxyxx", true)]
  #[case("uurcxstgmygtbstg", false)]
  #[case("ieodomkazucvgmuy", false)]
  fn test_criteria2(#[case] input: &str, #[case] expected: bool) {
    assert_eq!(
      criteria2(&&str::as_bytes(input)),
      expected,
      "input: {input}"
    );
  }
}
