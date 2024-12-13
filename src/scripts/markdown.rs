//! This module contains functions for updating markdown files.

use std::{
  fs::{read_to_string, File},
  io::Write,
  path::Path,
};

use crate::util::parse::ParseOps;

/// Adds/updates an entry for the given year to the README, in the solutions
/// table. If a benchmark duration is provided, it is added to the entry.
pub fn update_year_entry_in_readme(
  year: u32,
  is_complete: bool,
  duration_milliseconds: Option<f64>,
) {
  let readme_path = Path::new("README.md");
  let readme_content = read_to_string(readme_path).unwrap();

  let year_entry_index = format!("| [{year}](./src/y{year}/) |");
  let benchmark = if let Some(duration) = duration_milliseconds
    && is_complete
  {
    format!("{duration:.2} |")
  } else {
    "- |".to_string()
  };
  let year_entry = format!("{year_entry_index} {benchmark}");

  let mut modified_readme_content = vec![];

  if readme_content.contains(&year_entry_index) {
    for line in readme_content.lines() {
      if line.starts_with(&year_entry_index) {
        modified_readme_content.push(year_entry.clone());
      } else {
        modified_readme_content.push(line.to_string());
      }
    }
  } else {
    let mut is_in_solutions_section = false;
    let mut is_in_solutions_table = false;
    let mut did_insert_entry = false;

    for line in readme_content.lines() {
      if line == "## Solutions" {
        is_in_solutions_section = true;
      } else if line.starts_with('#') {
        is_in_solutions_section = false;
      }

      if is_in_solutions_table && !did_insert_entry {
        if let Some(line_year) = line.iter_unsigned::<u32>().next() {
          if line_year > year && !did_insert_entry {
            modified_readme_content.push(year_entry.clone());
            did_insert_entry = true;
          }
        }

        if !line.starts_with('|') && !did_insert_entry {
          is_in_solutions_section = false;
          modified_readme_content.push(year_entry.clone());
          did_insert_entry = true;
        }
      }

      is_in_solutions_table = line.starts_with('|') && is_in_solutions_section;

      modified_readme_content.push(line.to_string());
    }
  }

  let modified_readme_content: String = modified_readme_content.join("\n");
  let mut readme_file = File::create(readme_path).unwrap();
  write!(readme_file, "{modified_readme_content}").unwrap();
  println!("Updated README with entry for year '{year}'");
}

/// Adds/updates an entry for the given day to the year README, in the solutions
/// table. The title is used as the display name for the entry.
pub fn update_day_entry_in_year_readme(
  year: u32,
  day: u32,
  title: Option<&str>,
  is_complete: bool,
  duration_microseconds: Option<f64>,
) {
  let readme_path = Path::new("src").join(format!("y{year}")).join("README.md");
  let readme_content = read_to_string(&readme_path).unwrap();

  let day_entry_index = format!("| [{day:0>2}](./d{day:0>2}.rs) |");
  let benchmark = if let Some(duration) = duration_microseconds
    && is_complete
  {
    format!("{duration:.2} |")
  } else {
    "- |".to_string()
  };

  let mut modified_readme_content = vec![];

  if readme_content.contains(&day_entry_index) {
    for line in readme_content.lines() {
      if line.starts_with(&day_entry_index) {
        modified_readme_content
          .push(line.replace("- |", &benchmark).to_string());
      } else {
        modified_readme_content.push(line.to_string());
      }
    }
  } else {
    let day_entry = if let Some(title) = title {
      format!(
        "{day_entry_index} [{title}](https://adventofcode.com/{year}/day/{day}) | \
         {benchmark}",
      )
    } else {
      eprintln!("No title provided for day '{day:0>2}'");
      return;
    };

    let mut is_in_table = false;
    let mut did_insert_entry = false;

    for line in readme_content.lines() {
      if is_in_table && !did_insert_entry {
        if let Some(line_day) = line.iter_unsigned::<u32>().next() {
          if line_day > day && !did_insert_entry {
            modified_readme_content.push(day_entry.clone());
            did_insert_entry = true;
          }
        }

        if !line.starts_with('|') && !did_insert_entry {
          modified_readme_content.push(day_entry.clone());
          did_insert_entry = true;
        }
      }

      is_in_table = line.starts_with('|');
      modified_readme_content.push(line.to_string());
    }

    if !did_insert_entry {
      modified_readme_content.push(day_entry);
    }
  }

  let modified_readme_content: String = modified_readme_content.join("\n");
  let mut readme_file = File::create(&readme_path).unwrap();
  write!(readme_file, "{modified_readme_content}").unwrap();
  println!("Updated README for year '{year}' with entry for day '{day:0>2}'");
}
