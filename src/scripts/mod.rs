//! This module defines methods for various utility scripts.

#![allow(clippy::print_stdout)]
pub mod declarations;
pub mod markdown;

use std::path::Path;

use clap::Parser;

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
  markdown::add_year_to_readme(year);
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
  markdown::add_day_to_year_readme(year, day, &title);
}
