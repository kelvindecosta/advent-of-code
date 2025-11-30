//! # 1202 Program Alarm
//!
//! Given that:
//!
//! - the only operations in the program are addition and multiplication,
//! - all integers are positive
//!
//! we can deduce that the program is some linear function of the form
//! `ax + by + c`, where x and y are the noun and verb respectively.
//!
//! We first find the values of `a`, `b`, and `c` by evaluating the program at
//! `(0, 0)`, `(1, 0)`, and `(0, 1)`. With this, we are able to calculate part 1
//! by evaluating the program at `(12, 2)`.
//!
//! For part 2, we can use a two dimensional binary search to find the values of
//! `x` and `y` that satisfy the equation `ax + by + c = 19_690_720`.
//! Depending on the input, either `a` or `b` will be greater than the other, so
//! we can optimize the search by choosing the axis with the greater coefficient
//! first.
//!
//! > This solution was heavily inspired by [`maneatingape`'s solution](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2019/day02.rs).

use std::cmp::Ordering::{Equal, Less};

use super::intcode::IntcodeComputer;
use crate::util::parse::ParseOps;
type Input = [i32; 3];

pub fn evaluate_program(noun: i32, verb: i32, program: &[i32]) -> i32 {
  let mut computer = IntcodeComputer::new(program);
  computer.program[1] = noun;
  computer.program[2] = verb;
  computer.run()
}

pub fn parse(input: &str) -> Input {
  let program = input.iter_signed().collect::<Vec<_>>();
  let c = evaluate_program(0, 0, &program);
  let a = evaluate_program(1, 0, &program) - c;
  let b = evaluate_program(0, 1, &program) - c;

  [a, b, c]
}

pub fn p1([a, b, c]: &Input) -> i32 {
  12 * a + 2 * b + c
}

fn optimize(
  input: &Input,
  x1: i32,
  x2: i32,
  y1: i32,
  y2: i32,
  target: i32,
) -> Option<i32> {
  if x1 > x2 || y1 > y2 {
    return None;
  }

  let x_mid = (x1 + x2) / 2;
  let y_mid = (y1 + y2) / 2;
  let [a, b, c] = input;
  let optimize_x_first = usize::from(a > b);
  let result = a * x_mid + b * y_mid + c;

  match result.cmp(&target) {
    Equal => Some(100 * x_mid + y_mid),
    inequality => {
      let (x1_next, x2_next, y1_next, y2_next) = if inequality == Less {
        ([x1, x_mid + 1], [x2, x2], [y_mid + 1, y1], [y2, y2])
      } else {
        ([x1, x1], [x2, x_mid - 1], [y1, y1], [y_mid - 1, y2])
      };

      let try_optimize = |index: usize| {
        optimize(
          input,
          x1_next[index],
          x2_next[index],
          y1_next[index],
          y2_next[index],
          target,
        )
      };

      try_optimize(optimize_x_first)
        .or_else(|| try_optimize(optimize_x_first ^ 1))
    }
  }
}

#[allow(clippy::cast_possible_truncation)]
pub fn p2(input: &Input) -> i32 {
  optimize(input, 0, 99, 0, 99, 19_690_720).unwrap()
}
