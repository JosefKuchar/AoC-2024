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
| [Day 7](https://adventofcode.com/2024/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2024/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2024/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2024/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2024/day/11) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2024/day/12) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `51.2µs` | `81.2µs` |
| [Day 2](./src/bin/02.rs) | `173.6µs` | `374.8µs` |
| [Day 3](./src/bin/03.rs) | `358.7µs` | `323.8µs` |
| [Day 4](./src/bin/04.rs) | `5.7ms` | `109.3µs` |
| [Day 5](./src/bin/05.rs) | `1.5ms` | `35.6ms` |
| [Day 6](./src/bin/06.rs) | `611.2µs` | `178.9ms` |
| [Day 7](./src/bin/07.rs) | `1.7ms` | `86.1ms` |
| [Day 8](./src/bin/08.rs) | `55.8µs` | `136.5µs` |
| [Day 9](./src/bin/09.rs) | `942.7µs` | `122.7ms` |
| [Day 10](./src/bin/10.rs) | `530.9µs` | `284.2µs` |
| [Day 11](./src/bin/11.rs) | `238.7µs` | `12.7ms` |
| [Day 12](./src/bin/12.rs) | `7.5ms` | `8.2ms` |

**Total: 464.87ms**
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
