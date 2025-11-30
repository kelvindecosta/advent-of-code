//! # Sunny with a Chance of Asteroids
//!
//! Use this docstring to explain the problem and how it is solved.

use crate::{
  util::parse::ParseOps,
  y2019::intcode::{IntcodeComputer, State},
};

pub fn parse(input: &str) -> Vec<i32> {
  input.iter_signed().collect()
}

pub fn run(input: &[i32], value: i32) -> i32 {
  let mut computer = IntcodeComputer::new(input);
  computer.input(value);
  loop {
    match computer.execute() {
      State::Output(value) => {
        if value != 0 {
          break value;
        }
      }
      State::Halted(_) => {
        unreachable!("Should have received non-zero output already")
      }
    }
  }
}

pub fn p1(input: &[i32]) -> i32 {
  run(input, 1)
}

pub fn p2(input: &[i32]) -> i32 {
  run(input, 5)
}
