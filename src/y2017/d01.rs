//! # Inverse Captcha
//!
//! Parse the captcha as bytes, then compare each digit with the digit at the
//! same position with an offset. This is done by cloning the input and
//! rotating/shifting it by the offset, then zipping the two vectors together.

use crate::util::parse::ParseByte;

pub struct Captcha {
  digits: Vec<u8>,
}

impl Captcha {
  pub fn solve(&self, offset: usize) -> u32 {
    let mut compared = self.digits.clone();
    compared.rotate_left(offset);

    self
      .digits
      .iter()
      .zip(compared.iter())
      .filter_map(|(a, b)| (a == b).then_some(u32::from(a.to_decimal())))
      .sum()
  }
}

pub fn parse(input: &str) -> Captcha {
  Captcha {
    digits: input.trim().as_bytes().to_vec(),
  }
}

pub fn p1(input: &Captcha) -> u32 {
  input.solve(1)
}

pub fn p2(input: &Captcha) -> u32 {
  input.solve(input.digits.len() / 2)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("1122", 3)]
  #[case("1111", 4)]
  #[case("1234", 0)]
  #[case("91212129", 9)]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("1212", 6)]
  #[case("1221", 0)]
  #[case("123425", 4)]
  #[case("123123", 12)]
  #[case("12131415", 4)]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
