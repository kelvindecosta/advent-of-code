use std::{
  cmp::{max, Ordering, Reverse},
  collections::BinaryHeap,
  str::FromStr,
};

use eyre::Result;

#[derive(Clone, PartialEq, Eq)]
struct ElfInventory {
  food_calories: Vec<u32>,
}

impl ElfInventory {
  fn total_calories(&self) -> u32 {
    self.food_calories.iter().sum()
  }
}

impl PartialOrd for ElfInventory {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.total_calories().cmp(&other.total_calories()))
  }
}

impl Ord for ElfInventory {
  fn cmp(&self, other: &Self) -> Ordering {
    ElfInventory::partial_cmp(self, other).unwrap()
  }
}

impl FromStr for ElfInventory {
  type Err = eyre::Error;

  fn from_str(text: &str) -> Result<Self> {
    let food_calories = text
      .lines()
      .map(|line| {
        line
          .parse()
          .map_err(|e| eyre::eyre!("Failed to parse food calorie: {e}"))
      })
      .collect::<Result<Vec<_>>>()?;

    Ok(Self { food_calories })
  }
}

#[aoc(day01, part1)]
fn p1(input: &str) -> u32 {
  input
    .trim()
    .split("\n\n")
    .map(|group| group.parse().unwrap())
    .fold(None, |result, elf_inventory: ElfInventory| {
      Some(result.map_or(elf_inventory.clone(), |r| max(elf_inventory, r)))
    })
    .unwrap()
    .total_calories()
}

#[aoc(day01, part2)]
fn p2(input: &str) -> u32 {
  let elf_inventories: Vec<ElfInventory> = input
    .trim()
    .split("\n\n")
    .map(|group| group.parse().unwrap())
    .collect::<Vec<_>>();

  let mut top_three = BinaryHeap::new();

  for elf_inventory in elf_inventories {
    top_three.push(Reverse(elf_inventory.clone()));

    if top_three.len() > 3 {
      top_three.pop();
    }
  }

  top_three
    .iter()
    .map(|inventory| inventory.0.total_calories())
    .sum()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    24000
  )]
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(input), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    45000
  )]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(input), expected, "input: {input}");
  }
}
