//! # Rock Paper Scissors
//!
//! We define the `Shape` enum to represent the three possible plays in the game
//! of Rock Paper Scissors. Each `Round`, consisting of the opponent's play and
//! the player's play, is scored according to the rules of the game.
//!
//! For part 2, we calculate the `response` of the player's play to the
//! opponent's play in order to win/lose/draw.

use std::cmp::Ordering::{self, Equal, Greater, Less};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
  Rock,
  Paper,
  Scissors,
}
use Shape::{Paper, Rock, Scissors};

impl Shape {
  pub fn from_byte(byte: u8) -> Self {
    match byte {
      b'A' | b'X' => Rock,
      b'B' | b'Y' => Paper,
      b'C' | b'Z' => Scissors,
      _ => unreachable!("Not a valid play"),
    }
  }

  #[must_use]
  pub const fn response(&self, outcome: Ordering) -> Self {
    match (self, outcome) {
      (Rock, Less) | (Paper, Greater) => Scissors,
      (Paper, Less) | (Scissors, Greater) => Rock,
      (Scissors, Less) | (Rock, Greater) => Paper,
      _ => *self,
    }
  }
}

impl Ord for Shape {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Equal,
      (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Greater,
      _ => Less,
    }
  }
}

impl PartialOrd for Shape {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug)]
pub struct Round {
  theirs: Shape,
  mine: Shape,
}

impl Round {
  pub fn score(&self) -> u32 {
    let shape_score = match self.mine {
      Rock => 1,
      Paper => 2,
      Scissors => 3,
    };

    let outcome_score = match self.mine.cmp(&self.theirs) {
      Less => 0,
      Equal => 3,
      Greater => 6,
    };

    shape_score + outcome_score
  }
}

pub type Strategy = (u8, u8);

pub fn parse(input: &str) -> Vec<Strategy> {
  input
    .as_bytes()
    .array_chunks::<4>()
    .map(|line| (line[0], line[2]))
    .collect()
}

pub fn p1(input: &[Strategy]) -> u32 {
  input
    .iter()
    .map(|&(first, second)| {
      let theirs = Shape::from_byte(first);
      let mine = Shape::from_byte(second);
      Round { theirs, mine }.score()
    })
    .sum()
}

pub fn p2(input: &[Strategy]) -> u32 {
  input
    .iter()
    .map(|&(first, second)| {
      let theirs = Shape::from_byte(first);
      let mine = Shape::response(
        &theirs,
        match second {
          b'X' => Less,
          b'Y' => Equal,
          b'Z' => Greater,
          _ => unreachable!("Not a valid scenario"),
        },
      );
      Round { theirs, mine }.score()
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "A Y
B X
C Z
",
    15
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "A Y
B X
C Z
",
    12
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
