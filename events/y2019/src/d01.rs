#[must_use]
pub const fn calculate_fuel(mass: u32) -> u32 {
  (mass / 3) - 2
}

#[must_use]
pub fn calculate_recursive_fuel(mass: u32) -> u32 {
  let fuel = calculate_fuel(mass);
  if fuel <= (2 * 3) {
    fuel
  } else {
    fuel + calculate_recursive_fuel(fuel)
  }
}

#[aoc(day01, part1)]
fn p1(input: &[u32]) -> u32 {
  input.iter().map(|&mass| calculate_fuel(mass)).sum()
}

#[aoc(day01, part2)]
fn p2(input: &[u32]) -> u32 {
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
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(
      p1(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }

  #[rstest]
  #[case("14", 2)]
  #[case("1969", 966)]
  #[case("100756", 50346)]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(
      p2(
        input
          .lines()
          .map(|line| line.parse().unwrap())
          .collect::<Vec<_>>()
          .as_slice()
      ),
      expected,
      "input: {input}"
    );
  }
}
