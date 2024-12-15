//! # High-Entropy Passphrases
//!
//! For part 1, we use a `HashSet` to keep track of the words we've seen so far.
//! For part 2, we keep track of the frequency of each character in the word,
//! since anagrams have the same frequency of characters.

use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Vec<&str>> {
  input
    .trim()
    .lines()
    .map(|line| line.split_ascii_whitespace().collect())
    .collect()
}

pub fn p1(input: &[Vec<&str>]) -> usize {
  input
    .iter()
    .filter(|words| {
      let mut seen = HashSet::new();
      words.iter().all(|word| seen.insert(word))
    })
    .count()
}

pub fn p2(input: &[Vec<&str>]) -> usize {
  input
    .iter()
    .filter(|words| {
      let mut seen = HashSet::new();
      words.iter().all(|word| {
        // let mut chars = word.chars().collect::<Vec<_>>();
        // chars.sort_unstable();
        let mut freq = [0; 26];
        for c in word.bytes() {
          freq[(c - b'a') as usize] += 1;
        }
        seen.insert(freq)
      })
    })
    .count()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa",
    2
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio",
    3
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
