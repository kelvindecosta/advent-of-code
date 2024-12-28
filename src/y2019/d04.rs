//! # Secure Container
//!
//! The checking for an adjacent pair can be done by iterating over windows of
//! size 2 and mapping the result of the comparison of the two elements.
//!
//! For part 1, we simply check if any comparison was true.
//! For part 2, we check if there is at least one comparison that is true while
//! its neighbors are false.

use itertools::Itertools;

use crate::util::{parse::ParseOps, slice::SliceOps};

pub fn to_digits(mut num: u32) -> Vec<u32> {
  let mut digits = Vec::new();
  while num > 0 {
    digits.push(num % 10);
    num /= 10;
  }
  digits.reverse();
  digits
}

pub fn is_increasing(digits: &[u32]) -> bool {
  digits.windows(2).all(|w| w[0] <= w[1])
}

pub fn check_adjacent_pairs(digits: &[u32]) -> Vec<bool> {
  digits.windows(2).map(|w| w[0] == w[1]).collect()
}

pub fn get_next_non_decreasing(digits: &mut [u32]) {
  // Find the last occurrence of a digit that is not 9
  let mut i = digits.len() - 1;
  while i > 0 && digits[i] == 9 {
    i -= 1;
  }

  // Increment the digit
  let next_digit = digits[i] + 1;

  // Fill the rest of the digits with the next digit
  let digits_to_fill = i..digits.len();
  digits[digits_to_fill].fill(next_digit);
}

pub fn parse(input: &str) -> Vec<Vec<u32>> {
  let (start, end) = input.iter_unsigned().collect_tuple().unwrap();

  let mut digits = to_digits(start);

  let mut passwords = Vec::new();

  get_next_non_decreasing(&mut digits);

  loop {
    let num = digits.fold_decimal();
    if num > end {
      break;
    }

    passwords.push(digits.clone());
    get_next_non_decreasing(&mut digits);
  }

  passwords
}

pub fn criteria1(digits: &[u32]) -> bool {
  is_increasing(digits) && check_adjacent_pairs(digits).iter().any(|&b| b)
}

pub fn criteria2(digits: &[u32]) -> bool {
  if !is_increasing(digits) {
    return false;
  }
  let checks = check_adjacent_pairs(digits);

  checks[0] && !checks[1]
    || checks.windows(3).any(|w| !w[0] && w[1] && !w[2])
    || !checks[checks.len() - 2] && checks[checks.len() - 1]
}

pub fn p1(input: &[Vec<u32>]) -> usize {
  input.iter().filter(|&digits| criteria1(digits)).count()
}

pub fn p2(input: &[Vec<u32>]) -> usize {
  input.iter().filter(|&digits| criteria2(digits)).count()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(111_111, true)]
  #[case(223_450, false)]
  #[case(123_789, false)]
  fn test_criteria1(#[case] input: u32, #[case] expected: bool) {
    assert_eq!(criteria1(&to_digits(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(112_233, true)]
  #[case(123_444, false)]
  #[case(111_122, true)]
  fn test_criteria2(#[case] input: u32, #[case] expected: bool) {
    assert_eq!(criteria2(&to_digits(input)), expected, "input: {input}");
  }
}
