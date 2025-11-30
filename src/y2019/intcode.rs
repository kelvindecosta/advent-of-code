//! [Intcode] is the computer language used by the plot of the 2019 Advent of
//! Code. It is a simple language that uses a series of integers to perform
//! operations on a list of integers.
//!
//! [Intcode]: https://esolangs.org/wiki/Intcode

use std::collections::VecDeque;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// [Opcodes] for the Intcode computer.
///
/// [Opcodes]: https://esolangs.org/wiki/Intcode#Opcodes
#[derive(FromPrimitive)]
pub enum Opcode {
  Add = 1,
  Multiply = 2,
  Input = 3,
  Output = 4,
  JumpIfTrue = 5,
  JumpIfFalse = 6,
  LessThan = 7,
  Equals = 8,
  Halt = 99,
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
  Output(i32),
  Halted(i32),
}

/// The Intcode computer.
///
/// The computer has a program in its memory and a register for the program
/// counter, i.e., the instruction pointer.
///
/// It can also accept an input queue.
#[derive(Debug, Default)]
pub struct IntcodeComputer {
  pub program: Vec<i32>,
  pub ip: usize,
  pub input: VecDeque<usize>,
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
impl IntcodeComputer {
  pub fn new(program: &[i32]) -> Self {
    Self {
      program: program.to_vec(),
      ..Default::default()
    }
  }

  pub fn input(&mut self, value: i32) {
    self.input.push_back(value as usize);
  }

  pub fn address(&self, instruction: i32, offset: usize) -> usize {
    let index = self.ip + offset;
    let mode = (instruction / 10_i32.pow((offset as u32) + 1)) % 10;

    match mode {
      0 => self.program[index] as usize,
      1 => index,
      _ => unreachable!("Not a valid mode"),
    }
  }

  pub fn execute(&mut self) -> State {
    loop {
      let instruction = self.program[self.ip];
      let op = FromPrimitive::from_i32(instruction % 100).unwrap();

      match op {
        Opcode::Add => {
          let a = self.program[self.address(instruction, 1)];
          let b = self.program[self.address(instruction, 2)];
          let ox = self.address(instruction, 3);
          self.program[ox] = a + b;
          self.ip += 4;
        }
        Opcode::Multiply => {
          let a = self.program[self.address(instruction, 1)];
          let b = self.program[self.address(instruction, 2)];
          let ox = self.address(instruction, 3);
          self.program[ox] = a * b;
          self.ip += 4;
        }
        Opcode::Input => {
          let value = self.input.pop_front().unwrap();
          let ox = self.address(instruction, 1);
          self.program[ox] = value as i32;
          self.ip += 2;
        }
        Opcode::Output => {
          let a = self.program[self.address(instruction, 1)];
          self.ip += 2;
          break State::Output(a);
        }
        Opcode::JumpIfTrue => {
          let a = self.program[self.address(instruction, 1)];
          if a != 0 {
            self.ip = self.program[self.address(instruction, 2)] as usize;
          } else {
            self.ip += 3;
          }
        }
        Opcode::JumpIfFalse => {
          let a = self.program[self.address(instruction, 1)];
          if a == 0 {
            self.ip = self.program[self.address(instruction, 2)] as usize;
          } else {
            self.ip += 3;
          }
        }
        Opcode::LessThan => {
          let a = self.program[self.address(instruction, 1)];
          let b = self.program[self.address(instruction, 2)];
          let ox = self.address(instruction, 3);
          self.program[ox] = (a < b).into();
          self.ip += 4;
        }
        Opcode::Equals => {
          let a = self.program[self.address(instruction, 1)];
          let b = self.program[self.address(instruction, 2)];
          let ox = self.address(instruction, 3);
          self.program[ox] = (a == b).into();
          self.ip += 4;
        }
        Opcode::Halt => break State::Halted(self.program[0]),
      }
    }
  }

  pub fn run(&mut self) -> i32 {
    loop {
      match self.execute() {
        State::Output(_) => {}
        State::Halted(value) => break value,
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;
  use crate::util::parse::ParseOps;

  #[rstest]
  #[case("1,9,10,3,2,3,11,0,99,30,40,50", 3500)]
  fn test_intcode_computer_execute(
    #[case] instructions: &str,
    #[case] expected: i32,
  ) {
    let program = instructions.iter_signed().collect::<Vec<_>>();
    let mut computer = IntcodeComputer::new(&program);
    assert_eq!(computer.run(), expected);
  }

  #[rstest]
  #[case("1,0,0,0,99", &[2, 0, 0, 0, 99])]
  #[case("2,3,0,3,99", &[2, 3, 0, 6, 99])]
  #[case("2,4,4,5,99,0", &[2, 4, 4, 5, 99, 9801])]
  #[case("1,1,1,4,99,5,6,0,99", &[30, 1, 1, 4, 2, 5, 6, 0, 99])]
  #[case("1002,4,3,4,33", &[1002, 4, 3, 4, 99])]
  #[case("1101,100,-1,4,0", &[1101, 100, -1, 4, 99])]
  fn test_intcode_computer_final_state(
    #[case] instructions: &str,
    #[case] expected: &[i32],
  ) {
    let program = instructions.iter_signed().collect::<Vec<_>>();
    let mut computer = IntcodeComputer::new(&program);
    computer.run();
    assert_eq!(computer.program, expected);
  }

  #[rstest]
  #[case("3,0,4,0,99", 1337, 1337)]
  fn test_intcode_computer_input(
    #[case] instructions: &str,
    #[case] input: i32,
    #[case] expected: i32,
  ) {
    let program = instructions.iter_signed().collect::<Vec<_>>();
    let mut computer = IntcodeComputer::new(&program);
    computer.input(input);

    assert_eq!(computer.run(), expected);
  }

  #[rstest]
  // Position mode: equal to 8
  #[case("3,9,8,9,10,9,4,9,99,-1,8", 8, 1)]
  #[case("3,9,8,9,10,9,4,9,99,-1,8", 7, 0)]
  #[case("3,9,8,9,10,9,4,9,99,-1,8", 9, 0)]
  // Position mode: less than 8
  #[case("3,9,7,9,10,9,4,9,99,-1,8", 7, 1)]
  #[case("3,9,7,9,10,9,4,9,99,-1,8", 8, 0)]
  #[case("3,9,7,9,10,9,4,9,99,-1,8", 9, 0)]
  // Immediate mode: equal to 8
  #[case("3,3,1108,-1,8,3,4,3,99", 8, 1)]
  #[case("3,3,1108,-1,8,3,4,3,99", 7, 0)]
  #[case("3,3,1108,-1,8,3,4,3,99", 9, 0)]
  // Immediate mode: less than 8
  #[case("3,3,1107,-1,8,3,4,3,99", 7, 1)]
  #[case("3,3,1107,-1,8,3,4,3,99", 8, 0)]
  #[case("3,3,1107,-1,8,3,4,3,99", 9, 0)]
  fn test_comparison_operations(
    #[case] instructions: &str,
    #[case] input: i32,
    #[case] expected: i32,
  ) {
    let program = instructions.iter_signed().collect::<Vec<_>>();
    let mut computer = IntcodeComputer::new(&program);
    computer.input(input);

    match computer.execute() {
      State::Output(value) => assert_eq!(value, expected),
      State::Halted(_) => panic!("Expected output, got halt"),
    }
  }

  #[rstest]
  // Position mode: output 0 if input is 0, else output 1
  #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0)]
  #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 1, 1)]
  #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", -5, 1)]
  #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 42, 1)]
  // Immediate mode: output 0 if input is 0, else output 1
  #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0)]
  #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1, 1)]
  #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", -5, 1)]
  #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 42, 1)]
  fn test_jump_operations(
    #[case] instructions: &str,
    #[case] input: i32,
    #[case] expected: i32,
  ) {
    let program = instructions.iter_signed().collect::<Vec<_>>();
    let mut computer = IntcodeComputer::new(&program);
    computer.input(input);

    match computer.execute() {
      State::Output(value) => assert_eq!(value, expected),
      State::Halted(_) => panic!("Expected output, got halt"),
    }
  }

  #[rstest]
  // Output 999 if input < 8, 1000 if input == 8, 1001 if input > 8
  #[case(
    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,\
     1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,\
     1,46,98,99",
    7,
    999
  )]
  #[case(
    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,\
     1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,\
     1,46,98,99",
    8,
    1000
  )]
  #[case(
    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,\
     1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,\
     1,46,98,99",
    9,
    1001
  )]
  #[case(
    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,\
     1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,\
     1,46,98,99",
    0,
    999
  )]
  #[case(
    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,\
     1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,\
     1,46,98,99",
    100,
    1001
  )]
  fn test_larger_example(
    #[case] instructions: &str,
    #[case] input: i32,
    #[case] expected: i32,
  ) {
    let program = instructions.iter_signed().collect::<Vec<_>>();
    let mut computer = IntcodeComputer::new(&program);
    computer.input(input);

    match computer.execute() {
      State::Output(value) => assert_eq!(value, expected),
      State::Halted(_) => panic!("Expected output, got halt"),
    }
  }
}
