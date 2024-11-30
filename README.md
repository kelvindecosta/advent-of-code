# Advent of Code 🎄

This repository contains my solutions for the [Advent of Code](https://adventofcode.com/) challenges, written in Rust.

## Setup

Besides having the Rust toolchain installed, you will need the following tools:

- [`just`](https://github.com/casey/just)
- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
- [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli)

```bash
cargo install just cargo-generate aoc-cli
```

> [!TIP]
> To use this repository as a template for your own solutions, simply delete the `events` directory (which contain all my solutions).
> You may then run the `just make-event <year>` and `just make-puzzle <year> <day>` commands to scaffold the boilerplate code for the event and puzzles of a specific year.

### Authentication

Follow [the instructions provided in the `aoc-cli` repository](https://github.com/scarvalhojr/aoc-cli?tab=readme-ov-file#session-cookie-) to authenticate with the Advent of Code website using a session cookie.

## Usage

```bash
just
```

```
Available recipes:
    default
    download year day    # ⬇️ Download input for a specific day's puzzle
    format               # 👔 Format the codebase
    lint                 # 🧹 Lint the codebase
    make-event year      # 🎄 Scaffold boilerplate for a new year's event
    make-puzzle year day # 📅 Scaffold boilerplate for a new day's puzzle
    solve year day part  # 🧩 Execute the code for one part of a specific day's puzzle on the input
    submit year day part # 🚚 Send the answer for one part of a specific day's puzzle
    test year day        # 🧪 Test the code for a specific day's puzzle
```

## Solutions

Here are [my solutions](./events) grouped by year and day.

## Credits

- [Advent of Code](https://adventofcode.com/) - The fantastic website that hosts the challenges for free every year.
- [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli) - An awesome CLI tool to interact with the Advent of Code website.
- [`evenfurther/aoc`](https://github.com/evenfurther/aoc) - A neat crate that simplifies the creation of boilerplate code for these challenges.
- [`awesome-advent-of-code`](https://github.com/Bogdanp/awesome-advent-of-code) - A curated list of awesome resources related to the Advent of Code, without which I wouldn't have found the tools mentioned above.
