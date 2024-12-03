#[aoc(day01, part1)]
fn p1(input: &[u32]) -> u32 {
  input
    .iter()
    .skip(1)
    .enumerate()
    // Check if the current digit is greater than the previous digit
    .filter(|(index, &value)| input[*index] < value)
    .count() as u32
}

#[aoc(day01, part2)]
fn p2(input: &[u32]) -> u32 {
  let mut prev_sum = None;

  input
    .iter()
    .skip(2)
    .enumerate()
    // Check if the current sum is greater than the previous sum
    .filter(|(index, &value)| {
      let curr_sum = input[*index] + input[*index + 1] + value;
      let is_increasing = prev_sum.is_some_and(|prev_sum| prev_sum < curr_sum);
      prev_sum = Some(curr_sum);
      is_increasing
    })
    .count() as u32
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "199
200
208
210
200
207
240
269
260
263",
    7
  )]
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
  #[case(
    "199
200
208
210
200
207
240
269
260
263",
    5
  )]
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
