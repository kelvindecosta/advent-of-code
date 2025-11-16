//! # Giant Squid
//!
//! Use this docstring to explain the problem and how it is solved.
//!
//! For each board we determine the turn on which it would win, by finding the
//! earliest turn a row or column is fully marked. The score is then calculated
//! based on the unmarked numbers at that turn.
//!
//! For part 1 and 2 we find the board with the earliest and latest winning turn
//! respectively.

use std::array::from_fn;

use crate::util::parse::ParseOps;

const BOARD_SIZE: usize = 25;
const BINGO_CHECKS: [(usize, usize); 10] = [
  // (offset, step)
  // Rows
  (0, 1),
  (5, 1),
  (10, 1),
  (15, 1),
  (20, 1),
  // Columns
  (0, 5),
  (1, 5),
  (2, 5),
  (3, 5),
  (4, 5),
];

pub struct Board {
  turn: usize,
  score: usize,
}

pub fn parse(input: &str) -> Vec<Board> {
  let (draws_str, boards_str) = input.split_once("\n\n").unwrap();
  let boards: Vec<_> = boards_str.iter_unsigned().collect();

  let mut number_to_turn = vec![0; 100];
  let mut turn_to_number = vec![0; 100];

  for (turn, number) in draws_str.iter_unsigned().enumerate() {
    number_to_turn[number] = turn;
    turn_to_number[turn] = number;
  }

  boards
    .chunks_exact(BOARD_SIZE)
    .map(|board| {
      let turns: [usize; BOARD_SIZE] =
        from_fn(|index| number_to_turn[board[index]]);
      let winning_turn = BINGO_CHECKS
        .iter()
        .map(|&(offset, step)| {
          *turns
            .iter()
            .skip(offset)
            .step_by(step)
            .take(5)
            .max()
            .unwrap()
        })
        .min()
        .unwrap();
      let unmarked: usize = board
        .iter()
        .filter(|&&number| number_to_turn[number] > winning_turn)
        .sum();
      let winning_number = turn_to_number[winning_turn];
      Board {
        turn: winning_turn,
        score: unmarked * winning_number,
      }
    })
    .collect()
}

pub fn p1(input: &[Board]) -> usize {
  input.iter().min_by_key(|board| board.turn).unwrap().score
}

pub fn p2(input: &[Board]) -> usize {
  input.iter().max_by_key(|board| board.turn).unwrap().score
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
    4512
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
    1924
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
