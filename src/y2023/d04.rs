//! # Scratchcards
//!
//! For part 1, after having counted the number of winning numbers for each
//! card, we score them as successive powers of 2.
//!
//! For part 2, we start with one copy of each card.
//! Whenever we encounter a card with a `n` winning numbers, we add, for every
//! copy of this card, a copy of the next `n` cards.

use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<usize> {
  input
    .lines()
    .map(|line| {
      let mut found = [false; 100];
      let (win, have) = line.split_once('|').unwrap();
      win
        .iter_unsigned::<usize>()
        .skip(1)
        .for_each(|i| found[i] = true);
      have.iter_unsigned::<usize>().filter(|&i| found[i]).count()
    })
    .collect()
}

pub fn p1(input: &[usize]) -> usize {
  input.iter().map(|&n| (1 << n) >> 1).sum()
}

pub fn p2(input: &[usize]) -> usize {
  let mut copies: Vec<usize> = vec![1; input.len()];

  input.iter().enumerate().for_each(|(i, &n)| {
    (0..n).for_each(|j| copies[i + j + 1] += copies[i]);
  });

  copies.iter().sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    13
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    30
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
