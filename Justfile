default:
  @just -l

# 🏗️ Scaffold boilerplate for a new year or day's puzzle
scaffold year day="":
  #!/usr/bin/env bash
  set -euo pipefail
  if [ -z "{{day}}" ]; then
    just make-event {{year}}
  else
    just make-puzzle {{year}} {{day}}
  fi

# Scaffold boilerplate for a new year's event
[private]
@make-event year:
  cargo generate --path templates/year --name y{{year}} --destination src/y{{year}} --define year={{year}} --init > /dev/null
  ./scripts/scaffold-year.rs {{year}} > /dev/null

# Download input for a specific day's puzzle
[private]
@download year day:
  mkdir -p input/y{{year}}
  aoc download --quiet --year {{year}} --day {{day}} --input-only --overwrite --input-file input/y{{year}}/d{{day}}.txt

# Scaffold boilerplate for a new day's puzzle
[private]
make-puzzle year day:
  #!/usr/bin/env bash
  set -euo pipefail
  aoc_puzzle_title=$(aoc read --year {{year}} --day {{day}} 2>&1 |  awk -v day="{{day}}" '/^--- Day [0-9]+: / { sub(/^--- Day [0-9]+: /, ""); sub(/ ---$/, ""); print; exit }')
  cargo generate --path templates/day --name day --destination src/y{{year}} --define year={{year}} --define day={{day}} --define title="$aoc_puzzle_title" --init > /dev/null
  ./scripts/scaffold-day.rs {{year}} {{day}} "$aoc_puzzle_title" > /dev/null
  just download {{year}} {{day}}

# 🧪 Check if the solutions pass the base examples
test year="" day="":
  #!/usr/bin/env bash
  set -euo pipefail
  if [ -z "{{year}}" ]; then
    cargo test
  elif [ -z "{{day}}" ]; then
    cargo test "y{{year}}"
  else
    cargo test "y{{year}}::d$(printf '%02d' {{day}})"
  fi

# 🐞 Run the solutions with diagnostic messages
debug year="" day="" format="":
  #!/usr/bin/env bash
  set -euo pipefail
  args=()
  [ -n "{{year}}" ] && args+=(--year {{year}})
  [ -n "{{day}}" ] && args+=(--day {{day}})
  [ -n "{{format}}" ] && args+=(--format="{{format}}")
  cargo run -- "${args[@]}"

# 🧩 Execute the solutions and obtain answers in a formatted output
solve year="" day="" format="":
  #!/usr/bin/env bash
  set -euo pipefail
  args=()
  [ -n "{{year}}" ] && args+=(--year {{year}})
  [ -n "{{day}}" ] && args+=(--day {{day}})
  [ -n "{{format}}" ] && args+=(--format="{{format}}")
  cargo run --quiet -- "${args[@]}"

# 📤 Send the answer for one part of a specific day's puzzle
@submit year day part:
  just solve {{year}} {{day}} json | jq -r ".[0].part{{part}}" | xargs -I {} aoc submit --quiet --year {{year}} --day {{day}} {{part}} {}

# 📊 Measure the performance of the solutions
bench year="" day="" part="":
  #!/usr/bin/env bash
  set -euo pipefail
  if [ -z "{{year}}" ]; then
    cargo bench --bench benchmark
  elif [ -z "{{day}}" ]; then
    cargo bench --bench benchmark "y{{year}}"
  elif [ -z "{{part}}" ]; then
    cargo bench --bench benchmark "y{{year}}_d$(printf '%02d' {{day}})"
  else
    cargo bench --bench benchmark "y{{year}}_d$(printf '%02d' {{day}})_p{{part}}"
  fi
  ./scripts/update-documentation-with-benchmarks.rs > /dev/null
  just format

# 🧹 Lint the codebase
@lint *FLAGS:
  cargo clippy --quiet {{FLAGS}}

# 👔 Format the codebase
@format:
  cargo +nightly fmt --all
  prettier --write "**/*.md" > /dev/null
