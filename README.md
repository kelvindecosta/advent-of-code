# Advent of Code 🎄

This repository contains my solutions for the [Advent of Code](https://adventofcode.com/) challenges, written in Rust.

## Setup

Besides having the nightly channel of the Rust toolchain installed, you will need the following tools:

- [`just`](https://github.com/casey/just) - A handy command runner for project-specific tasks
- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) - A utility to quickly scaffold boilerplate code
- [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli) - A CLI tool to interact with the Advent of Code website

> [!TIP]
> To use this repository as a template for your own solutions, simply delete the sub-directories in the `src/` directory that match the `y{year}` pattern and remove the modules from the `lib.rs` file.
> You may then run the `just scaffold <year>` and `just scaffold <year> <day>` commands to scaffold the boilerplate code for the event and puzzles of a specific year.

### Authentication

Follow [the instructions provided in the `aoc-cli` repository](https://github.com/scarvalhojr/aoc-cli?tab=readme-ov-file#session-cookie-) to authenticate with the Advent of Code website using a session cookie.

## Usage

```bash
just
```

```
Available recipes:
    bench year="" day="" part=""   # 📊 Measure the performance of the solutions
    debug year="" day="" format="" # 🐞 Run the solutions with diagnostic messages
    default
    format                         # 👔 Format the codebase
    lint *FLAGS                    # 🧹 Lint the codebase
    scaffold year day=""           # 🏗️ Scaffold boilerplate for a new year or day's puzzle
    solve year="" day="" format="" # 🧩 Execute the solutions and obtain answers in a formatted output
    submit year day part           # 📤 Send the answer for one part of a specific day's puzzle
    test year="" day=""            # 🧪 Check if the solutions pass the base examples
```

## Solutions

|                 Year | Benchmark (ms) |
| -------------------: | -------------: |
| [2015](./src/y2015/) |              - |
| [2016](./src/y2016/) |              - |
| [2017](./src/y2017/) |              - |
| [2018](./src/y2018/) |              - |
| [2019](./src/y2019/) |              - |
| [2020](./src/y2020/) |              - |
| [2021](./src/y2021/) |              - |
| [2022](./src/y2022/) |              - |
| [2023](./src/y2023/) |              - |
| [2024](./src/y2024/) |              - |

## Credits

- [Advent of Code](https://adventofcode.com/) - The fantastic website that hosts the challenges for free every year.

- [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli) - An awesome CLI tool to interact with the Advent of Code website.

- [`maneatingape/advent-of-code-rust`](https://github.com/maneatingape/advent-of-code-rust) - An excellent set of solutions to the Advent of Code challenges in Rust.

  > The project structure and patterns used in this repository are heavily inspired by this project.
  > I've also borrowed some of the utilities from there.

- [`evenfurther/aoc`](https://github.com/evenfurther/aoc) - A neat crate that simplifies the creation of boilerplate code for these challenges.

  > I initially opted for this crate and generated multiple crates for each year, but I disliked having to use a workspace, which is why I refactored away from it. This project introduced me to [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

- [`awesome-advent-of-code`](https://github.com/Bogdanp/awesome-advent-of-code) - A curated list of awesome resources related to the Advent of Code, without which I wouldn't have found the tools mentioned above.
