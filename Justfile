default:
  @just --list

# 🏗️  Scaffold boilerplate for a new year's event
@make-event year:
  cargo generate --path templates/year --name y{{year}} --destination src/y{{year}} --define year={{year}} --init > /dev/null
  ./scripts/scaffold-year.rs {{year}} > /dev/null

# 📥 Download input for a specific day's puzzle
@download year day:
  mkdir -p input/y{{year}}
  aoc download --quiet --year {{year}} --day {{day}} --input-only --overwrite --input-file input/y{{year}}/d{{day}}.txt

# 📅 Scaffold boilerplate for a new day's puzzle
make-puzzle year day:
  #!/usr/bin/env bash
  set -euo pipefail
  aoc_puzzle_title=$(aoc read --year {{year}} --day {{day}} 2>&1 |  awk -v day="{{day}}" '/^--- Day [0-9]+: / { sub(/^--- Day [0-9]+: /, ""); sub(/ ---$/, ""); print; exit }')
  cargo generate --path templates/day --name day --destination src/y{{year}} --define year={{year}} --define day={{day}} --define title="$aoc_puzzle_title" --init > /dev/null
  ./scripts/scaffold-day.rs {{year}} {{day}} "$aoc_puzzle_title" > /dev/null
  just download {{year}} {{day}}

# 🧪 Check if the solutions pass the base examples
@test *FLAGS:
  cargo test {{FLAGS}}

# 🐞 Run the solutions with diagnostic messages
@debug *FLAGS:
  cargo run -- {{FLAGS}}

# 🧩 Execute the solutions and obtain answers in a formatted output
@solve *FLAGS:
  cargo run --quiet -- {{FLAGS}}

# 📤 Send the answer for one part of a specific day's puzzle
@submit year day part:
  just solve --year {{year}} --day {{day}} --format="json" | jq -r ".[0].part{{part}}" | xargs -I {} aoc submit --quiet --year {{year}} --day {{day}} {{part}} {}

# 📊 Measure the performance of the solutions
@bench *FLAGS:
  cargo bench --bench benchmark {{FLAGS}}

# 🧹 Lint the codebase
@lint *FLAGS:
  cargo clippy --quiet {{FLAGS}}

# 👔 Format the codebase
@format:
  cargo +nightly fmt --all
  prettier --write "**/*.md" > /dev/null
