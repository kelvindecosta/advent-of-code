use std::str::FromStr;

use lazy_static::lazy_static;
use ndarray::{s, Array, ArrayBase, Dim, OwnedRepr, Zip};

#[derive(Debug, Clone, Copy, Eq)]
pub enum LetterMatch {
  Exact(char),
  Any,
}

impl TryFrom<char> for LetterMatch {
  type Error = ();

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '*' => Ok(Self::Any),
      _ => Ok(Self::Exact(value)),
    }
  }
}

impl PartialEq for LetterMatch {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Exact(a), Self::Exact(b)) => a == b,
      (Self::Any, _) | (_, Self::Any) => true,
    }
  }
}

pub struct LetterGrid {
  grid: ArrayBase<OwnedRepr<LetterMatch>, Dim<[usize; 2]>>,
}

impl FromStr for LetterGrid {
  type Err = ();

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let lines = text.trim().lines();
    let num_rows = lines.clone().count();
    let num_cols = lines.clone().next().unwrap().len();
    let letters = lines
      .flat_map(|line| {
        line
          .chars()
          .map(|c| c.try_into().unwrap())
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();

    let grid = Array::from_shape_vec((num_rows, num_cols), letters)
      .unwrap()
      .to_owned();

    Ok(Self { grid })
  }
}

pub struct Criteria {
  #[allow(clippy::type_complexity)]
  kernels: Vec<ArrayBase<OwnedRepr<LetterMatch>, Dim<[usize; 2]>>>,
}

impl Criteria {
  #[must_use]
  pub fn occurrences(&self, grid: &LetterGrid) -> u32 {
    self
      .kernels
      .iter()
      .map(|kernel| {
        grid
          .grid
          .windows(kernel.dim())
          .into_iter()
          .map(|window| {
            let matches = Zip::from(window)
              .and(kernel)
              .map_collect(|&w, &k| w == k)
              .into_iter()
              .collect::<Vec<_>>();

            // There must be matches and all of them must be true
            !matches.is_empty() && matches.iter().all(|&b| b)
          })
          .filter(|&b| b)
          .count()
      })
      .sum::<usize>() as u32
  }
}

#[must_use]
pub fn define_p1_criteria() -> Criteria {
  let word = "XMAS";
  let word_len = word.len();
  let hor_for_mat = Array::from_shape_vec(
    (1, word_len),
    word
      .chars()
      .map(|c| c.try_into().unwrap())
      .collect::<Vec<_>>(),
  )
  .unwrap()
  .to_owned();

  let hor_bak_mat = hor_for_mat.slice(s![.., ..;-1]).to_owned();
  let ver_for_mat = hor_for_mat.clone().reversed_axes().to_owned();
  let ver_bak_mat = ver_for_mat.slice(s![..;-1, ..]).to_owned();

  // Leading diagonal grid
  let ldg_for_mat = Array::from_shape_vec(
    (word_len, word_len),
    hor_for_mat
      .iter()
      .enumerate()
      .flat_map(|(i, &c)| {
        (0..word_len).map(move |j| if i == j { c } else { LetterMatch::Any })
      })
      .collect::<Vec<_>>(),
  )
  .unwrap()
  .to_owned();
  let ldg_bak_mat = ldg_for_mat.slice(s![..;-1, ..;-1]).to_owned();

  // Trailing diagonal grid
  let tdg_for_mat = ldg_for_mat.slice(s![..;-1, ..]).to_owned();
  let tdg_bak_mat = tdg_for_mat.slice(s![..;-1, ..;-1]).to_owned();

  Criteria {
    kernels: vec![
      hor_for_mat,
      hor_bak_mat,
      ver_for_mat,
      ver_bak_mat,
      ldg_for_mat,
      ldg_bak_mat,
      tdg_for_mat,
      tdg_bak_mat,
    ],
  }
}

fn define_p2_criteria() -> Criteria {
  let base_mat = Array::from_shape_vec(
    (3, 3),
    "M*S*A*M*S"
      .chars()
      .map(|c| c.try_into().unwrap())
      .collect::<Vec<_>>(),
  )
  .unwrap()
  .to_owned();

  let mut kernels = vec![];

  // Swap the edges of the diagonals, resulting in 4 kernels
  for (i, j) in (0..=1).flat_map(|i| (0..=1).map(move |j| (i, j))) {
    let mut transformed_mat = base_mat.clone();

    // Swap leading diagonal
    if i == 1 {
      transformed_mat.swap((0, 0), (2, 2));
    }

    // Swap trailing diagonal
    if j == 1 {
      transformed_mat.swap((0, 2), (2, 0));
    }

    kernels.push(transformed_mat);
  }

  Criteria { kernels }
}

lazy_static! {
  pub static ref CRITERIA_P1: Criteria = define_p1_criteria();
  pub static ref CRITERIA_P2: Criteria = define_p2_criteria();
}

#[aoc(day04, part1)]
fn p1(input: &str) -> u32 {
  CRITERIA_P1.occurrences(&input.parse().unwrap())
}

#[aoc(day04, part2)]
fn p2(input: &str) -> u32 {
  CRITERIA_P2.occurrences(&input.parse().unwrap())
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
    4
  )]
  #[case(
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    18
  )]
  #[case(
    "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
",
    18
  )]
  fn test_p1_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(input), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
    9
  )]
  fn test_p2_examples(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(input), expected, "input: {input}");
  }
}
