//! This module contains functions for adding module declarations to Rust files.

use std::{
  error::Error,
  fs::{read_to_string, File},
  io::Write,
  path::Path,
};

/// Adds a module declaration to the given file.
pub fn add_module_declaration(
  module: &str,
  library_path: &Path,
) -> Result<(), Box<dyn Error>> {
  let declaration = format!("pub mod {module};");

  // Check if the declaration already exists
  let mut library_content = read_to_string(library_path)?;
  if library_content.contains(&declaration) {
    eprintln!("The declaration for module '{module}' already exists");
    return Ok(());
  }

  // Append the declaration to the library file
  library_content.push_str(&declaration);
  library_content.push('\n');
  let mut library_file = File::create(library_path)?;
  write!(library_file, "{library_content}")?;
  println!("Added declaration for module '{module}'");
  Ok(())
}
