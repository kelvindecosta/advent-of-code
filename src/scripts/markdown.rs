//! This module contains functions for updating markdown files.

use std::{
  fs::{read_to_string, File},
  io::Write,
  path::Path,
};

use crate::util::parse::ParseOps;

/// Adds an entry for the given year to the README, in the solutions table.
pub fn add_year_to_readme(year: u32) {
  let readme_path = Path::new("README.md");
  let readme_content = read_to_string(readme_path).unwrap();

  let year_entry_index = format!("| [{year}](./src/y{year}/) |");
  let year_entry = format!("{year_entry_index} (incomplete) |");

  if readme_content.contains(&year_entry_index) {
    eprintln!("The entry for year '{year}' already exists in the README");
    return;
  }

  let mut is_in_solutions_section = false;
  let mut is_in_solutions_table = false;
  let mut did_insert_entry = false;

  let mut modified_readme_content = vec![];

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

  let modified_readme_content: String = modified_readme_content.join("\n");
  let mut readme_file = File::create(readme_path).unwrap();
  write!(readme_file, "{modified_readme_content}").unwrap();
  println!("Updated README with entry for year '{year}'");
}

/// Adds an entry for the given day to the year README, in the solutions table.
/// The title is used as the display name for the entry.
pub fn add_day_to_year_readme(year: u32, day: u32, title: &str) {
  let readme_path = Path::new("src").join(format!("y{year}")).join("README.md");
  let readme_content = read_to_string(&readme_path).unwrap();

  let day_entry_index = format!("| [{day:0>2}](./d{day:0>2}.rs) |");
  let day_entry = format!(
    "{day_entry_index} [{title}](https://adventofcode.com/{year}/day/{day}) | \
     (incomplete) |"
  );

  if readme_content.contains(&day_entry_index) {
    eprintln!(
      "The entry for day '{day:0>2}' already exists in the year README"
    );
    return;
  }

  let mut is_in_table = false;
  let mut did_insert_entry = false;

  let mut modified_readme_content = vec![];

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

  let modified_readme_content: String = modified_readme_content.join("\n");
  let mut readme_file = File::create(&readme_path).unwrap();
  write!(readme_file, "{modified_readme_content}").unwrap();
  println!("Updated README with entry for day '{day:0>2}'");
}
