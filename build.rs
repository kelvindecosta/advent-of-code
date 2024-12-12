//! This build script generates files with information on which puzzles have
//! been attempted in the codebase.
use std::{
  env,
  fs::{read_dir, File},
  io::Write,
  path::Path,
};

/// Create a file with the given content
fn make_file(out_dir: &str, filename: &str, content: &str) {
  let path = Path::new(out_dir).join(filename);
  let mut file = File::create(&path).unwrap();
  write!(file, "{content}")
    .unwrap_or_else(|_| panic!("Failed to write {filename}"));
}

/// Generate the `solvers.rs` file
///
/// This file contains a static vector of `Solver` structs that are generated
/// using the `solver` macro.
///
/// It is included in the `main.rs` file.
fn generate_solvers(out_dir: &str, puzzles: &[(u32, u32)]) {
  let mut generated_code = String::new();

  // Generate the code for a static vector of solver macro invocations
  if !puzzles.is_empty() {
    generated_code
      .push_str("lazy_static! {\nstatic ref SOLVERS: Vec<Solver> = vec![\n");

    for &puzzle in puzzles {
      generated_code
        .push_str(&format!("solver!(y{:>4}, d{:0>2}),\n", puzzle.0, puzzle.1));
    }

    generated_code.push_str("];\n}\n");
  }

  make_file(out_dir, "solvers.rs", &generated_code);
}

/// Generate the `benchmarks.rs` file
///
/// This file contains the benchmark macro invocations for each puzzle.
///
/// It is included in the `benches/benchmark.rs` file.
fn generate_benchmarks(out_dir: &str, puzzles: &[(u32, u32)]) {
  let mut generated_code = String::new();

  generated_code.push_str(
    r#"
fn aoc_bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("aoc");

  "#
    .trim(),
  );

  for &(year, day) in puzzles {
    generated_code.push_str(
      format!(
        r#"
let data = read_to_string(
  Path::new("input")
    .join("y{year:>4}")
    .join("d{day:0>2}")
    .with_extension("txt"),
).unwrap();

group.bench_with_input(
  BenchmarkId::new("y{year:>4}_d{day:0>2}_parse", ""),
  &data,
  |b, d| {{
    b.iter(|| y{year:>4}::d{day:0>2}::parse(d));
  }},
);

let input = y{year:>4}::d{day:0>2}::parse(&data);

group.bench_with_input(
  BenchmarkId::new("y{year:>4}_d{day:0>2}_p1", ""),
  &input,
  |b, i| {{
    b.iter(|| y{year:>4}::d{day:0>2}::p1(i));
  }},
);

group.bench_with_input(
  BenchmarkId::new("y{year:>4}_d{day:0>2}_p2", ""),
  &input,
  |b, i| {{
    b.iter(|| y{year:>4}::d{day:0>2}::p2(i));
  }},
);
    "#
      )
      .trim(),
    );
  }

  generated_code.push_str(
    r"
  group.finish();
}"
    .trim(),
  );

  make_file(out_dir, "benchmarks.rs", &generated_code);
}

fn main() {
  let mut puzzles = Vec::new();

  // Read the src directory
  let src_dir = Path::new("src");
  for entry in read_dir(src_dir).expect("Failed to read src directory") {
    let entry = entry.expect("Failed to read entry");
    let path = entry.path();

    if path.is_dir() {
      if let Some(year_mod) = path.file_name().and_then(|name| name.to_str()) {
        // Check for those directories in the src directory that start with 'y'
        // and parse as a year
        let year = year_mod[1..].parse::<u32>();
        if !year_mod.starts_with('y') || year.is_err() {
          continue;
        }
        let year = year.unwrap();

        for day_entry in read_dir(&path).expect("Failed to read year directory")
        {
          let day_entry = day_entry.expect("Failed to read day entry");
          let day_path = day_entry.path();

          if let Some(day_mod) =
            day_path.file_stem().and_then(|name| name.to_str())
          {
            // Check for those directories in the year directory that start with
            // 'd' and parse as a day
            let day = day_mod[1..].parse::<u32>();
            if !day_mod.starts_with('d') || day.is_err() {
              continue;
            }
            let day = day.unwrap();

            puzzles.push((year, day));
          }
        }
      }
    }
  }

  // Create the files
  let out_dir =
    env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");

  generate_solvers(&out_dir, &puzzles);
  generate_benchmarks(&out_dir, &puzzles);
}
