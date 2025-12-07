//! # Laboratories
//!
//! While iterating through the rows in the manifold, we keep track of the
//! number of timelines (starting with just 1 at the 'S' position):
//!
//! Whenever we encounter a '^', we increment the number of timelines on the
//! left and right (if they are within the bounds of the manifold). The number
//! of timelines in the current space is reset to 0.
//!
//! For part 1 we count the number of encounters and for part 2 we sum the
//! number of timelines in all spaces.

pub fn parse(input: &str) -> (usize, usize) {
  let manifold = input.trim().lines().map(str::as_bytes).collect::<Vec<_>>();
  let width = manifold[0].len();

  let mut input_beams = manifold[0]
    .iter()
    .map(|&c| usize::from(c == b'S'))
    .collect::<Vec<_>>();
  let mut output_breams = vec![0; width];

  let mut splits = 0;

  for row in manifold {
    for (index, &timelines) in input_beams.iter().enumerate() {
      if timelines == 0 {
        continue;
      }

      if row[index] == b'^' {
        splits += 1;
        if index > 0 {
          output_breams[index - 1] += timelines;
        }
        if (index + 1) < width {
          output_breams[index + 1] += timelines;
        }
      } else {
        output_breams[index] += timelines;
      }
    }

    (input_beams, output_breams) = (output_breams, input_beams);
    output_breams.fill(0);
  }

  (splits, input_beams.iter().sum())
}

pub const fn p1(input: &(usize, usize)) -> usize {
  input.0
}

pub const fn p2(input: &(usize, usize)) -> usize {
  input.1
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
    21
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
    40
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
