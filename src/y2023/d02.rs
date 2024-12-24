//! # Cube Conundrum
//!
//! For each game we compute the maximum of each color of cubes that could have
//! been in the bag.

use crate::util::parse::ParseOps;

#[derive(Default, Debug)]
pub struct Game {
  red: u32,
  green: u32,
  blue: u32,
}

pub fn parse(input: &str) -> Vec<Game> {
  input
    .lines()
    .map(|line| {
      line
        .split_ascii_whitespace()
        .array_chunks::<2>()
        .skip(1)
        .fold(Game::default(), |mut game, [amount, color]| {
          let amount = amount.unsigned();
          match color.as_bytes()[0] {
            b'r' => game.red = game.red.max(amount),
            b'g' => game.green = game.green.max(amount),
            b'b' => game.blue = game.blue.max(amount),
            _ => unreachable!(),
          };
          game
        })
    })
    .collect()
}

pub fn p1(input: &[Game]) -> usize {
  input
    .iter()
    .enumerate()
    .filter_map(|(index, game)| {
      (game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .then_some(index + 1)
    })
    .sum()
}

pub fn p2(input: &[Game]) -> u32 {
  input
    .iter()
    .map(|game| game.red * game.green * game.blue)
    .sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    8
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    2286
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
