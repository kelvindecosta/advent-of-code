//! # Binary Diagnostic
//!
//! We parse each line as a `u16`.
//! To determine the most common bit in a column, we:
//! - mask out the bit at the index
//! - sum all bits
//! - if the sum is greater than the length of the column minus the sum, the
//!   most common bit is 1, otherwise 0.
//!
//! To get the least common bit, we simply invert the most common bit.

type Input = (Vec<u16>, usize);
pub fn parse(input: &str) -> Input {
  let lines = input.trim().lines().collect::<Vec<_>>();
  let width = lines[0].len();

  let data = lines
    .iter()
    .map(|line| u16::from_str_radix(line, 2).unwrap())
    .collect();

  (data, width)
}

pub const fn bit_at(num: u16, width: usize, index: usize) -> u16 {
  let offset = width - index - 1;
  (num >> offset) & 1
}

pub fn bit_column(data: &[u16], width: usize, index: usize) -> Vec<u16> {
  data.iter().map(|&row| bit_at(row, width, index)).collect()
}

#[allow(clippy::cast_possible_truncation)]
pub fn most_common_bit(bits: &[u16]) -> u32 {
  let total = bits.iter().sum::<u16>();
  u32::from((bits.len() as u16) - total <= total)
}

pub fn p1((data, width): &Input) -> u32 {
  let mut gamma = 0u32;
  let mut epsilon = 0u32;

  for index in 0..*width {
    let bits = bit_column(data, *width, index);
    let mcb = most_common_bit(&bits);

    gamma = (gamma << 1) | mcb;
    epsilon = (epsilon << 1) | (mcb ^ 1);
  }

  gamma * epsilon
}

pub fn p2((data, width): &Input) -> u32 {
  let mut oxy_rating_nums = data.clone();
  let mut co2_rating_nums = data.clone();

  let mut index = 0;
  while oxy_rating_nums.len() > 1 || co2_rating_nums.len() > 1 {
    if oxy_rating_nums.len() > 1 {
      let oxy_bits = bit_column(&oxy_rating_nums, *width, index);
      let oxy_mcb = most_common_bit(&oxy_bits);
      oxy_rating_nums
        .retain(|&num| u32::from(bit_at(num, *width, index)) == oxy_mcb);
    }

    if co2_rating_nums.len() > 1 {
      let co2_bits = bit_column(&co2_rating_nums, *width, index);
      let co2_lcb = most_common_bit(&co2_bits) ^ 1;
      co2_rating_nums
        .retain(|&num| u32::from(bit_at(num, *width, index)) == co2_lcb);
    }

    index += 1;
  }

  u32::from(oxy_rating_nums[0]) * u32::from(co2_rating_nums[0])
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    198
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    230
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
