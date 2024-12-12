#![allow(clippy::print_stdout)]
use std::{
  fs::read_to_string,
  path::{Path, PathBuf},
};

use aoc::util::parse::ParseOps;
use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use prettytable::{
  cell,
  format::{FormatBuilder, LinePosition, LineSeparator},
  row, table,
};
use serde::Serialize;

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
enum OutputFormat {
  #[default]
  Table,
  Json,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  year: Option<u32>,

  #[arg(short, long)]
  day: Option<u32>,

  #[arg(short, long, default_value = "table")]
  format: OutputFormat,
}

struct Solver {
  year: u32,
  day: u32,
  path: PathBuf,
  wrapper: fn(String) -> (String, String),
}

macro_rules! solver {
  ($year:tt, $day:tt) => {{
    let year = stringify!($year);
    let day = stringify!($day);
    let path = Path::new("input")
      .join(year)
      .join(day)
      .with_extension("txt");

    let wrapper = |data: String| {
      use aoc::$year::$day::*;

      let input = parse(&data);
      let part1 = p1(&input);
      let part2 = p2(&input);

      (part1.to_string(), part2.to_string())
    };

    Solver {
      year: year.unsigned(),
      day: day.unsigned(),
      path,
      wrapper,
    }
  }};
}

include!(concat!(env!("OUT_DIR"), "/solvers.rs"));

#[derive(Serialize)]
struct Solution {
  year: u32,
  day: u32,
  part1: String,
  part2: String,
}

fn main() {
  let Args { year, day, format } = Args::parse();

  // Filter solvers
  let solvers = SOLVERS
    .iter()
    .filter(|solver| year.is_none_or(|y: u32| y == solver.year))
    .filter(|solver| day.is_none_or(|d: u32| d == solver.day));

  let mut solutions = Vec::new();

  for Solver {
    year,
    day,
    path,
    wrapper,
  } in solvers
  {
    let data = read_to_string(path).unwrap();
    let (part1, part2) = wrapper(data);
    solutions.push(Solution {
      year: *year,
      day: *day,
      part1,
      part2,
    });
  }

  match format {
    OutputFormat::Table => {
      let mut table = table!();

      table.add_row(row![
        cell!("Year"),
        cell!("Day"),
        cell!("Part 1"),
        cell!("Part 2"),
      ]);

      for Solution {
        year,
        day,
        part1,
        part2,
      } in solutions
      {
        table.add_row(row![
          cell!(year),
          cell!(format!("{:>3}", format!("{day:0>2}"))),
          part1,
          part2,
        ]);
      }

      table.set_format(
        FormatBuilder::new()
          .column_separator('│')
          .borders('│')
          .separators(
            &[LinePosition::Top],
            LineSeparator::new('─', '┬', '┌', '┐'),
          )
          .separators(
            &[LinePosition::Intern],
            LineSeparator::new('─', '┼', '├', '┤'),
          )
          .separators(
            &[LinePosition::Bottom],
            LineSeparator::new('─', '┴', '└', '┘'),
          )
          .padding(3, 3)
          .build(),
      );
      table.printstd();
    }
    OutputFormat::Json => {
      println!("{}", serde_json::to_string_pretty(&solutions).unwrap());
    }
  }
}
