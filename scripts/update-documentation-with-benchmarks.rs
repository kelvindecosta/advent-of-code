#!/usr/bin/env cargo -Zscript --quiet
---
[package]
edition="2021"

[dependencies]
aoc = { path = "../."}
---

//! This script updates the documentation with the benchmarks for each puzzle.
//! It reads from the `target/criterion` directory and updates:
//! 
//! - the root `README.md` file with the total of the average duration for all solutions for each year.
//! - each year's `README.md` file with the total of the average duration for each function required to solve each day's puzzle.

use aoc::scripts::update_documentation_with_benchmarks;

fn main() {
  update_documentation_with_benchmarks();
}
