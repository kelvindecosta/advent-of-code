//! # Alchemical Reduction
//!
//! Since the ASCII numbers for the lowercase and the uppercase letters of the
//! same type always differ by 32, we can check if two units are of the same
//! type and have differing cases by a simple XOR.
//!
//! For part 1, we simply reduce the polymer once and compute the result's
//! length.
//!
//! For part 2, from the reduction in part 1, we find the minimum length polymer
//! when removing all the units of the same type, once for each letter in the
//! alphabet.

use itertools::Itertools;

pub fn reduce(polymer: impl Iterator<Item = u8>) -> Vec<u8> {
  let mut inert = Vec::with_capacity(polymer.try_len().unwrap_or(10_000));

  for unit in polymer {
    if let Some(reactant) = inert.pop() {
      // If they don't match in type keep both
      if unit ^ reactant != 32 {
        inert.push(reactant);
        inert.push(unit);
      }
    } else {
      inert.push(unit);
    }
  }

  inert
}

pub fn parse(input: &str) -> Vec<u8> {
  reduce(input.trim().bytes())
}

pub const fn p1(input: &[u8]) -> usize {
  input.len()
}

pub fn p2(input: &[u8]) -> usize {
  (b'a'..=b'z')
    .map(|letter| {
      reduce(input.iter().copied().filter(|&b| b | 32 != letter)).len()
    })
    .min()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("aA", 0)]
  #[case("abBA", 0)]
  #[case("abAB", 4)]
  #[case("aabAAB", 6)]
  #[case("dabAcCaCBAcCcaDA", 10)]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("dabAcCaCBAcCcaDA", 4)]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
