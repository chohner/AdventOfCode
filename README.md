# Advent of Code

[Advent Of Code](https://adventofcode.com/) is an Advent calendar of small programming puzzles. This repo contains my solutions in Rust.

## Usage

Please refer to the [official Rust documentation](https://www.rust-lang.org/tools/install) on how to install Rust on your machine.

This repo uses workspaces. However, the binaries current can't be executed from root due to the relative paths of data files.

Instead, navigate to the corresponding year and run a specific day:

```bash
> cd aoc2022
> cargo run --bin 1

# Finished dev [unoptimized + debuginfo] target(s) in 0.04s
#      Running `target/debug/1`
# Reading file ./src/bin/1/input.txt
# Part 1 took 26.497µs to find: <ANSWER 1>
# Part 2 took 114.937µs to find: <ANSWER 2>
```

## Project structure

- `src/lib.rs` for shared code (reading, printing, benchmarking)
- `src/bin/{day}/main.rs` for code
- `src/bin/{day}/input.txt` for daily input
