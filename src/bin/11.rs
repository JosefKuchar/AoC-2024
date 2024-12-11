use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn solve(input: &str, iterations: usize) -> Option<u64> {
    let mut stones: HashMap<u64, u64> = input
        .split_whitespace()
        .map(|x| (x.parse::<u64>().unwrap(), 1))
        .collect();
    for _ in 0..iterations {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for stone in stones.iter() {
            let new = if *stone.0 == 0 {
                vec![1]
            } else {
                let stone_str = stone.0.to_string();
                if stone_str.len() % 2 == 0 {
                    vec![
                        stone_str[0..stone_str.len() / 2].parse::<u64>().unwrap(),
                        stone_str[(stone_str.len() / 2)..].parse::<u64>().unwrap(),
                    ]
                } else {
                    vec![stone.0 * 2024]
                }
            };
            for n in new.iter() {
                *new_stones.entry(*n).or_insert(0) += stone.1;
            }
        }
        stones = new_stones;
    }
    Some(stones.iter().map(|x| x.1).sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
