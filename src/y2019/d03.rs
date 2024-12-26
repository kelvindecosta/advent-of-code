//! # Crossed Wires
//!
//! We use a `BTreeMap` to track the horizontal and vertical segments of the
//! first wire. As we trace the steps of the second wire, we check for
//! intersections with segments of the first wire in the opposite direction.
//!
//! > This solution was heavily inspired by [`maneatingape`'s solution](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2019/day03.rs).

use std::{cmp::minmax, collections::BTreeMap};

use itertools::Itertools;

use crate::util::{
  parse::ParseOps,
  point::{Point, ORIGIN},
};

struct Segment {
  start: Point,
  end: Point,
  cumulative_distance: i32,
}

type Answers = (i32, i32);

pub fn parse(input: &str) -> Answers {
  let (first_wire, second_wire) = input
    .trim()
    .lines()
    .map(|line| {
      let directions = line.bytes().filter(u8::is_ascii_alphabetic);
      let steps = line.iter_signed::<i32>();

      directions.zip(steps).collect::<Vec<_>>()
    })
    .collect_tuple()
    .unwrap();

  // Track vertical and horizontal segments of the wires
  let mut vertical_segments = BTreeMap::new();
  let mut horizontal_segments = BTreeMap::new();

  // Trace the steps of the first wire
  let mut start = ORIGIN;
  let mut cumulative_distance = 0;

  for (direction, length) in first_wire {
    let end = start + Point::from(direction) * length;
    let segment = Segment {
      start,
      end,
      cumulative_distance,
    };

    if start.x == end.x {
      vertical_segments.insert(start.x, segment);
    } else {
      horizontal_segments.insert(start.y, segment);
    }

    start = end;
    cumulative_distance += length;
  }

  // Trace the steps of the second wire, checking for intersections.
  let mut start = ORIGIN;
  let mut cumulative_distance = 0;
  let mut shortest_manhattan = i32::MAX;
  let mut min_delay = i32::MAX;

  for (direction, distance) in second_wire {
    let end = start + Point::from(direction) * distance;

    let potential_intersections = match direction {
      // Moving vertically, check for intersections with horizontal segments.
      b'U' | b'D' => {
        let [min_y, max_y] = minmax(start.y, end.y);
        horizontal_segments
          .range(min_y..=max_y)
          .map(|(&y, segment)| (segment, Point::new(start.x, y)))
          .collect::<Vec<_>>()
      }
      // Moving horizontally, check for intersections with vertical segments.
      b'R' | b'L' => {
        let [min_x, max_x] = minmax(start.x, end.x);
        vertical_segments
          .range(min_x..=max_x)
          .map(|(&x, segment)| (segment, Point::new(x, start.y)))
          .collect::<Vec<_>>()
      }
      _ => unreachable!(),
    };

    // Ignore intersections at the origin
    let potential_intersections = potential_intersections
      .iter()
      .filter(|&(_, candidate)| candidate.manhattan(ORIGIN) != 0);

    let intersects = |segment: &Segment, candidate: Point| {
      // Distance from the start to the candidate must be less than that to the
      // end and the candidate must be in the same direction as the end.
      segment.start.manhattan(candidate) < segment.start.manhattan(segment.end)
        && segment.start.signum(candidate) == segment.start.signum(segment.end)
    };

    for (segment, candidate) in potential_intersections {
      if intersects(segment, *candidate) {
        shortest_manhattan =
          shortest_manhattan.min(candidate.manhattan(ORIGIN));

        // Delay is the sum of the cumulative distances of the two segments and
        // the distance from each path to the intersection.
        min_delay = min_delay.min(
          cumulative_distance
            + candidate.manhattan(start)
            + segment.cumulative_distance
            + candidate.manhattan(segment.start),
        );
      }
    }

    start = end;
    cumulative_distance += distance;
  }

  (shortest_manhattan, min_delay)
}

pub const fn p1(input: &Answers) -> i32 {
  input.0
}

pub const fn p2(input: &Answers) -> i32 {
  input.1
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "R8,U5,L5,D3
U7,R6,D4,L4",
    6
  )]
  #[case(
    "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
    159
  )]
  #[case(
    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    135
  )]
  fn test_p1(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "R8,U5,L5,D3
U7,R6,D4,L4",
    30
  )]
  #[case(
    "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
    610
  )]
  #[case(
    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    410
  )]
  fn test_p2(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
