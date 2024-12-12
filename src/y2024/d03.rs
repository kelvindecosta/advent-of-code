//! # Mull It Over
//!
//! We iterate over the input string and parse the instructions as soon as they
//! are encountered.
//!
//! For the multiplication, we ensure that the numbers are separate by a comma
//! and enclosed in parentheses. No other characters are allowed.
//!
//! The other instructions are parsed by simply matching the string (as bytes).

type Answers = (u32, u32);

fn parse_number(memory: &[u8], index: &mut usize) -> u32 {
  let mut number = 0;
  while memory[*index].is_ascii_digit() {
    number = number * 10 + u32::from(memory[*index] - b'0');
    *index += 1;
  }
  number
}

fn parse_instruction(
  memory: &[u8],
  index: &mut usize,
  instruction: &[u8],
) -> bool {
  if memory[*index..].starts_with(instruction) {
    *index += instruction.len();
    true
  } else {
    false
  }
}

pub fn parse(input: &str) -> Answers {
  let memory = input.as_bytes();

  let mut index = 0;
  let mut enabled = true;

  let mut sum = 0;
  let mut cond_sum = 0;

  while index < memory.len() {
    // If the current position cannot be `mul`, `do` or `don't`, then move ahead
    if !(memory[index] == b'm' || memory[index] == b'd') {
      index += 1;
    }

    // Multiplication
    if memory[index..].starts_with(b"mul(") {
      index += 4;
      let a = parse_number(memory, &mut index);
      if memory[index] != b',' {
        continue;
      }
      index += 1;
      let b = parse_number(memory, &mut index);
      if memory[index] != b')' {
        continue;
      }
      index += 1;

      let product = a * b;
      sum += product;
      if enabled {
        cond_sum += product;
      }
    }
    // Do
    else if parse_instruction(memory, &mut index, b"do()") {
      enabled = true;
    }
    // Don't
    else if parse_instruction(memory, &mut index, b"don't()") {
      enabled = false;
    } else {
      index += 1;
    }
  }

  (sum, cond_sum)
}

pub const fn p1(input: &Answers) -> u32 {
  input.0
}

pub const fn p2(input: &Answers) -> u32 {
  input.1
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    161
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    48
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
