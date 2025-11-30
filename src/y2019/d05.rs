//! # Sunny with a Chance of Asteroids
//!
//! We expand on the [`Intcode`] computer we developed on [`2019/02`] and check
//! for the first non-zero output value before the program halts.

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
