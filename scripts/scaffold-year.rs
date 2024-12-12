#!/usr/bin/env cargo -Zscript --quiet
---
[package]
edition="2021"

[dependencies]
aoc = { path = "../."}
---

//! This script assists in scaffolding the boilerplate for a new year's event.
//!
//! Given a year, it:
//! - adds a module declaration for the year to the `src/lib.rs` file
//! - updates the `README.md` file with with a link to the new year's module

use aoc::scripts::scaffold_year;

fn main() {
  scaffold_year();
}
