#![allow(unstable_features)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::too_many_lines)]

use std::{fs::read_to_string, path::Path};

use aoc::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

include!(concat!(env!("OUT_DIR"), "/benchmarks.rs"));

criterion_group!(benches, aoc_bench);
criterion_main!(benches);
