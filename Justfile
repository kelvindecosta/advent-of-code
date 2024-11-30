default:
  @just --list

# 🎄 Scaffold boilerplate for a new year's event
@make-event year:
  cargo generate --path templates/year --name y{{year}} --destination events --define year={{year}}

# ⬇️  Download input for a specific day's puzzle
@download year day:
  aoc download --year {{year}} --day {{day}} --input-only --overwrite --input-file events/y{{year}}/input/d{{day}}.txt

# 📅 Scaffold boilerplate for a new day's puzzle
@make-puzzle year day:
  cargo generate --path templates/day --name day --destination events/y{{year}}/src --define year={{year}} --define day={{day}} --init
  ./scripts/declare-day-module.sh {{year}} {{day}}
  @just download {{year}} {{day}}

# 🧪 Test the code for a specific day's puzzle 
@test year day:
  cargo test --package y{{year}} d{{day}}

# 🧩 Execute the code for one part of a specific day's puzzle on the input
@solve year day part:
  cargo run --quiet --bin y{{year}} -- --day {{day}} --part {{part}} --input events/y{{year}}/input/d{{day}}.txt | awk -F': ' '{print $2}' | xargs

# 🚚 Send the answer for one part of a specific day's puzzle
@submit year day part:
  @just solve {{year}} {{day}} {{part}} | xargs -I {} aoc submit --year {{year}} --day {{day}} {{part}} {} --debug

# 🧹 Lint the codebase
@lint:
  cargo clippy --all-targets --all-features -- -D warnings

# 👔 Format the codebase
@format:
  cargo +nightly fmt --all
