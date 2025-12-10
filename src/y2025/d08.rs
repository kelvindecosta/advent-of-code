//! # Playground
//!
//! As we compute the score (squared distance between two boxes) for each
//! possible connection, we slot each connection into buckets based on their
//! scores. This is done to speed up the process of sorting them.
//!
//! Initially, each box is considered its own circuit.
//! This is treated as a [disjoint-set data structure](https://en.wikipedia.org/wiki/Disjoint-set_data_structure), for which we define `find` and `union` operations.
//! These `Circuit`s are sets containing the indices of the boxes comprising
//! them.
//!
//! We iterate over the connections, after having sorted them based on their
//! scores. The process of committing to these connections involves performing a
//! `union` on their boxes.
//!
//! For part 1, we simply find the three connections with the largest scores,
//! once we have reached a limit number of connections. For part 2, we find the
//! connection that completes the cycle, forming one `Circuit` with all the
//! boxes.
//!
//! > This solution was heavily inspired by [`maneatingape`'s solution](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2025/day08.rs).

use std::collections::BinaryHeap;

use crate::util::{
  parse::ParseOps,
  thread::{spawn_parallel_iterator, ParIter},
};

type Box = [usize; 3];

type Connection = (u16, u16, usize);

struct Circuit {
  parent: usize,
  size: usize,
}

const BUCKETS: usize = 5;
const SIZE: usize = 100_000_000;

#[allow(clippy::cast_possible_truncation)]
fn worker(boxes: &[Box], iter: ParIter<'_, usize>) -> Vec<Vec<Connection>> {
  let mut buckets = vec![vec![]; BUCKETS];

  // For each combination of boxes...
  for &i in iter {
    let v1 = boxes[i];

    for (j, &v2) in boxes.iter().enumerate().skip(i + 1) {
      // ...calculate the score (distance square)
      let dx = v1[0].abs_diff(v2[0]);
      let dy = v1[1].abs_diff(v2[1]);
      let dz = v1[2].abs_diff(v2[2]);
      let score = dx * dx + dy * dy + dz * dz;

      let index = (score / SIZE).min(BUCKETS - 1);
      buckets[index].push((i as u16, j as u16, score));
    }
  }

  buckets
}

fn find(set: &mut [Circuit], mut x: usize) -> usize {
  while set[x].parent != x {
    let parent = set[x].parent;
    (x, set[x].parent) = (parent, set[parent].parent);
  }

  x
}

fn union(set: &mut [Circuit], mut x: usize, mut y: usize) -> usize {
  x = find(set, x);
  y = find(set, y);

  if x != y {
    if set[x].size < set[y].size {
      (x, y) = (y, x);
    }

    set[y].parent = x;
    set[x].size += set[y].size;
  }

  set[x].size
}

pub fn parse(input: &str) -> (usize, usize) {
  let boxes: Vec<_> =
    input.iter_unsigned::<usize>().array_chunks::<3>().collect();

  let indices: Vec<_> = (0..boxes.len()).collect();

  let mut buckets = vec![vec![]; BUCKETS];

  for network in spawn_parallel_iterator(&indices, |iter| worker(&boxes, iter))
  {
    for (bucket, connections) in buckets.iter_mut().zip(network) {
      bucket.push(connections);
    }
  }

  // Consider each of the boxes as their own circuit
  let mut circuits: Vec<_> = (0..boxes.len())
    .map(|i| Circuit { parent: i, size: 1 })
    .collect();

  let mut result1 = 0;
  let mut result2 = 0;
  let limit: usize = if boxes.len() == 1000 { 1000 } else { 10 };

  // Take the connection with the lowest score, after having merged the buckets
  for (count, (i, j, ..)) in buckets
    .iter()
    .flat_map(|connections| {
      // Sort the connections in the bucket based on their scores
      let mut merged = connections.concat();
      merged.sort_unstable_by_key(|&(.., score)| score);
      merged
    })
    .enumerate()
  {
    let (i, j) = (i as usize, j as usize);

    // Attempt to connect the boxes.
    // Check if this connection forms the a complete circuit with all the boxes
    // (a cycle)
    if union(&mut circuits, i, j) == boxes.len() {
      result2 = boxes[i][0] * boxes[j][0];
    }

    // For part 1, we check if we've reached the limit...
    if count == limit - 1 {
      let mut heap =
        BinaryHeap::from(circuits.iter().map(|c| c.size).collect::<Vec<_>>());

      // ...and find the three connections with the greatest score
      result1 = (0..3).filter_map(|_| heap.pop()).product();
    }

    if result1 != 0 && result2 != 0 {
      break;
    }
  }

  (result1, result2)
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
    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
    40
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "162,817,812
  57,618,57
  906,360,560
  592,479,940
  352,342,300
  466,668,158
  542,29,236
  431,825,988
  739,650,466
  52,470,668
  216,146,977
  819,987,18
  117,168,530
  805,96,715
  346,949,466
  970,615,88
  941,993,340
  862,61,35
  984,92,344
  425,690,689",
    25272
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
