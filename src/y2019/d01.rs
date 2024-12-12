//! # The Tyranny of the Rocket Equation
//!
//! For part 1, we simply calculate the fuel for each module and sum them up.
//! For part 2, we calculate the fuel recursively until the fuel required is
//! less than or equal to 8, since any mass that requires less than 8 fuel will
//! not require any additional fuel.

use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<u32> {
  input.iter_unsigned().collect()
}

pub const fn calculate_fuel(mass: u32) -> u32 {
  (mass / 3).saturating_sub(2)
}

pub fn calculate_recursive_fuel(mass: u32) -> u32 {
  let fuel = calculate_fuel(mass);

  fuel
    + if fuel <= 8 {
      0
    } else {
      calculate_recursive_fuel(fuel)
    }
}

pub fn p1(input: &[u32]) -> u32 {
  input.iter().map(|&mass| calculate_fuel(mass)).sum()
}

pub fn p2(input: &[u32]) -> u32 {
  input
    .iter()
    .map(|&mass| calculate_recursive_fuel(mass))
    .sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("12", 2)]
  #[case("14", 2)]
  #[case("1969", 654)]
  #[case("100756", 33583)]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("14", 2)]
  #[case("1969", 966)]
  #[case("100756", 50346)]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
