advent_of_code::solution!(7);

use itertools::Itertools;
use rayon::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Clone)]
enum Operation {
    Concat,
    Add,
    Mul,
}

fn solve(input: &str, skip: usize) -> Option<u64> {
    Some(
        input
            .par_lines()
            .map(|line| {
                let mut parts = line.split(": ");
                let result = parts.next().unwrap().parse::<u64>().unwrap();
                let numbers = parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                (result, numbers)
            })
            .filter(|(result, numbers)| {
                (0..numbers.len() - 1)
                    .map(|_| Operation::iter().skip(skip))
                    .multi_cartesian_product()
                    .any(|c| {
                        let mut c = c.into_iter();
                        numbers.iter().copied().reduce(|acc, n| {
                            let op = c.next().unwrap();
                            match op {
                                Operation::Add => acc + n,
                                Operation::Mul => acc * n,
                                Operation::Concat => {
                                    let mut spacer = 1;
                                    while n / spacer > 0 {
                                        spacer *= 10;
                                    }
                                    acc * spacer + n
                                }
                            }
                        }) == Some(*result)
                    })
            })
            .map(|(result, _)| result)
            .sum::<u64>(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
