<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2024

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2024 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2024/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2024/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2024/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2024/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2024/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2024/day/6) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 7](./src/bin/07.rs) | `4.8ms` | `254.2ms` |

**Total: 259.00ms**
<!--- benchmarking table --->

---

## Usage

### ➡️ Scaffold a day

```sh
cargo scaffold <day>
```

### ➡️ Download input for a day

```sh
cargo download <day>
```

### ➡️ Run solutions for a day

```sh
cargo solve <day>
```

#### Submitting solutions

Append the `--submit <part>` option to the `solve` command to submit your solution for checking.

### ➡️ Run all solutions

```sh
cargo all
```

### ➡️ Benchmark your solutions

```sh
cargo time <day> [--all] [--store]
```

### ➡️ Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### ➡️ Read puzzle description

```sh
cargo read <day>
```

### ➡️ Scaffold, download & read the current aoc day

During december, the `today` shorthand command can be used to:

-   scaffold a solution for the current day
-   download its input
-   and read the puzzle

in one go.

```sh
cargo today
```
