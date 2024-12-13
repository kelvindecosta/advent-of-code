//! This module defines methods for various utility scripts.

#![allow(clippy::print_stdout)]
pub mod benchmark;
pub mod declarations;
pub mod markdown;

use std::path::Path;

use benchmark::Benchmark;
use clap::Parser;
use itertools::Itertools;
use markdown::{update_day_entry_in_year_readme, update_year_entry_in_readme};

#[derive(Parser, Debug)]
#[clap(version)]
struct ScaffoldYearArgs {
  year: u32,
}

pub fn scaffold_year() {
  let ScaffoldYearArgs { year } = ScaffoldYearArgs::parse();
  declarations::add_module_declaration(
    &format!("y{year}"),
    &Path::new("src").join("lib.rs"),
  )
  .unwrap();
  markdown::update_year_entry_in_readme(year, false, None);
}

#[derive(Parser, Debug)]
#[clap(version)]
struct ScaffoldDayArgs {
  year: u32,
  day: u32,
  title: String,
}

pub fn scaffold_day() {
  let ScaffoldDayArgs { year, day, title } = ScaffoldDayArgs::parse();
  declarations::add_module_declaration(
    &format!("d{day:0>2}"),
    &Path::new("src").join(format!("y{year}")).join("mod.rs"),
  )
  .unwrap();
  markdown::update_day_entry_in_year_readme(
    year,
    day,
    Some(&title),
    false,
    None,
  );
}

pub fn update_documentation_with_benchmarks() {
  let benchmarks = Benchmark::load_all();

  for (year, benchmarks_in_year) in &benchmarks.iter().chunk_by(|b| b.year) {
    let mut year_duration_milliseconds = 0.0;
    let mut day_count = 0;

    for (day, benchmarks_in_day) in &benchmarks_in_year
      .collect::<Vec<_>>()
      .iter()
      .chunk_by(|b| b.day)
    {
      let day_duration_microseconds = benchmarks_in_day
        .collect::<Vec<_>>()
        .iter()
        .map(|b| b.duration_nanoseconds / 1_000.0)
        .sum::<f64>();

      update_day_entry_in_year_readme(
        year,
        day,
        None,
        true,
        Some(day_duration_microseconds),
      );

      day_count += 1;
      year_duration_milliseconds += day_duration_microseconds / 1_000.0;
    }

    update_year_entry_in_readme(
      year,
      day_count == 25,
      Some(year_duration_milliseconds),
    );
  }
}
