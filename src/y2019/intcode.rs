//! [Intcode] is the computer language used by the plot of the 2019 Advent of
//! Code. It is a simple language that uses a series of integers to perform
//! operations on a list of integers.
//!
//! [Intcode]: https://esolangs.org/wiki/Intcode

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// [Opcodes] for the Intcode computer.
///
/// [Opcodes]: https://esolangs.org/wiki/Intcode#Opcodes
#[derive(FromPrimitive)]
pub enum Opcode {
  Add = 1,
  Multiply = 2,
  Halt = 99,
}

/// The Intcode computer.
///
/// The computer has a memory of integers and an instruction pointer.
#[derive(Debug, Default)]
pub struct IntcodeComputer {
  pub memory: Vec<i32>,
  pub ip: usize,
}

impl IntcodeComputer {
  pub fn new(program: &[i32]) -> Self {
    Self {
      memory: program.to_vec(),
      ..Default::default()
    }
  }

  #[allow(clippy::cast_sign_loss)]
  pub fn get_address(&self, index: usize) -> usize {
    self.memory[index] as usize
  }

  pub fn get_operand(&self, index: usize) -> i32 {
    self.memory[self.get_address(index)]
  }

  pub fn set(&mut self, index: usize, value: i32) {
    let io = self.get_address(index);
    self.memory[io] = value;
  }

  pub fn execute(&mut self) -> i32 {
    loop {
      let op = FromPrimitive::from_i32(self.memory[self.ip]).unwrap();

      match op {
        Opcode::Add => {
          let a = self.get_operand(self.ip + 1);
          let b = self.get_operand(self.ip + 2);
          self.set(self.ip + 3, a + b);
          self.ip += 4;
        }
        Opcode::Multiply => {
          let a = self.get_operand(self.ip + 1);
          let b = self.get_operand(self.ip + 2);
          self.set(self.ip + 3, a * b);
          self.ip += 4;
        }
        Opcode::Halt => break self.memory[0],
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
  fn test_intcode_computer_execute(#[case] input: &str, #[case] expected: i32) {
    let program = input.iter_signed().collect::<Vec<_>>();
    let mut computer = IntcodeComputer::new(&program);
    assert_eq!(computer.execute(), expected);
  }

  #[rstest]
  #[case("1,0,0,0,99", &[2, 0, 0, 0, 99])]
  #[case("2,3,0,3,99", &[2, 3, 0, 6, 99])]
  #[case("2,4,4,5,99,0", &[2, 4, 4, 5, 99, 9801])]
  #[case("1,1,1,4,99,5,6,0,99", &[30, 1, 1, 4, 2, 5, 6, 0, 99])]
  fn test_intcode_computer_final_state(
    #[case] input: &str,
    #[case] expected: &[i32],
  ) {
    let program = input.iter_signed().collect::<Vec<_>>();
    let mut computer = IntcodeComputer::new(&program);
    computer.execute();
    assert_eq!(computer.memory, expected);
  }
}
