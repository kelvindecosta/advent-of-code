//! This module contains utilities for parsing `criterion` benchmark data.

use std::{fs::read_to_string, path::PathBuf};

use glob::glob;
use serde_json::Value;

use crate::util::parse::ParseOps;

/// Represents a function that has been benchmarked.
pub enum BenchmarkedFunction {
  Parse,
  Part1,
  Part2,
}

/// Represents a benchmark result.
pub struct Benchmark {
  pub year: u32,
  pub day: u32,
  pub function: BenchmarkedFunction,
  pub duration_nanoseconds: f64,
}

impl Benchmark {
  /// Loads a benchmark from the given path.
  pub fn from_path(benchmark_path: &PathBuf) -> Self {
    let benchmark_name = benchmark_path
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .file_stem()
      .unwrap()
      .to_str()
      .unwrap();

    let numbers = benchmark_name.iter_unsigned().collect::<Vec<u32>>();
    let year = numbers[0];
    let day = numbers[1];

    let function =
      numbers
        .get(2)
        .map_or(BenchmarkedFunction::Parse, |part| match part {
          1 => BenchmarkedFunction::Part1,
          2 => BenchmarkedFunction::Part2,
          _ => panic!("Invalid part number: {part}"),
        });

    let benchmark_json: Value =
      serde_json::from_str(&read_to_string(benchmark_path).unwrap()).unwrap();

    let benchmark_measure = benchmark_json
      .get("mean")
      .or_else(|| benchmark_json.get("point_estimate"))
      .unwrap();

    let duration_nanoseconds = benchmark_measure
      .get("point_estimate")
      .unwrap()
      .as_f64()
      .unwrap();

    Self {
      year,
      day,
      function,
      duration_nanoseconds,
    }
  }

  /// Loads all benchmarks.
  pub fn load_all() -> Vec<Self> {
    let base_dir = PathBuf::from("target").join("criterion").join("aoc");

    if !base_dir.exists() {
      eprintln!("No benchmarks found");
      return Vec::new();
    }

    glob(
      base_dir
        .join("**")
        .join("new")
        .join("estimates")
        .with_extension("json")
        .to_str()
        .unwrap(),
    )
    .unwrap()
    .map(|entry| Self::from_path(&entry.unwrap()))
    .collect()
  }
}
