//! # Rucksack Reorganization
//!
//! For part 1, we split each line into its two halves and find the common
//! letter.
//!
//! For part 2, we chunk lines into groups of 3 and then find the common
//! letter.

pub fn parse(input: &str) -> Vec<&str> {
  input.trim().lines().collect()
}

pub fn common(groups: &[&[u8]]) -> u8 {
  let mut seen = [0; 256];

  for (n, group) in groups.iter().enumerate() {
    // The last group should simply return the first common item
    if n == groups.len() - 1 {
      return group
        .iter()
        .find_map(|&b| (seen[b as usize] == groups.len() - 1).then_some(b))
        .unwrap();
    }

    // Consider an item to have been seen, only if it is the first
    // time for that group
    group.iter().for_each(|&b| {
      if seen[b as usize] == n {
        seen[b as usize] += 1;
      }
    });
  }

  unreachable!("Should have found the common letter")
}

pub fn item_priority(item: u8) -> u32 {
  u32::from(match item {
    b'a'..=b'z' => item - b'a' + 1,
    b'A'..=b'Z' => item - b'A' + 27,
    _ => unreachable!("Not a valid letter"),
  })
}

pub fn p1(input: &[&str]) -> u32 {
  input
    .iter()
    .map(|line| {
      let bytes = line.as_bytes();
      item_priority(common(&[
        &bytes[0..bytes.len() / 2],
        &bytes[bytes.len() / 2..],
      ]))
    })
    .sum()
}

pub fn p2(input: &[&str]) -> u32 {
  input
    .iter()
    .map(|line| line.as_bytes())
    .array_chunks::<3>()
    .map(|chunk| item_priority(common(&chunk)))
    .sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    157
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    70
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
