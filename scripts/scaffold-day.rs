#!/usr/bin/env cargo -Zscript --quiet
---
[package]
edition="2021"

[dependencies]
aoc = { path = "../."}
---

//! This script assists in scaffolding the boilerplate for a new day's puzzle.
//!
//! Given a year and day, it:
//! - adds a module declaration for the day's puzzle to the year's module file
//! - updates the year's `README.md` file with with a link to the new day's module

use aoc::scripts::scaffold_day;

fn main() {
  scaffold_day();
}
