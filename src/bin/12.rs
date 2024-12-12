use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::prelude::connected_components;

advent_of_code::solution!(12);

fn in_bounds(map: &[Vec<char>], pos: (isize, isize)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as isize && pos.1 < map[0].len() as isize
}

fn neighbours(map: &[Vec<char>], pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut res = Vec::new();
    let own = map[pos.0 as usize][pos.1 as usize];
    for dir in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if in_bounds(map, next) {
            if map[next.0 as usize][next.1 as usize] == own {
                res.push(next);
            }
        }
    }
    res
}

fn solve(input: &str, func: fn(&HashSet<(isize, isize)>, &[Vec<char>]) -> usize) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut starts = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            starts.push((i as isize, j as isize));
        }
    }

    Some(
        connected_components(&starts, |&pos| neighbours(&map, pos))
            .iter()
            .map(|group| func(&group, &map))
            .sum::<usize>() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, |group, map| {
        group.len()
            * group
                .iter()
                .map(|pos| 4 - neighbours(&map, *pos).len())
                .sum::<usize>()
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, |group, map| {
        let mut sides: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
        group.iter().for_each(|pos| {
            let neighbours = neighbours(&map, *pos);
            for dir in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next = (pos.0 + dir.0, pos.1 + dir.1);
                if !neighbours.contains(&next) {
                    sides.entry(*(dir)).or_default().push(next);
                }
            }
        });
        group.len()
            * sides
                .iter()
                .map(|(key, parts)| {
                    let sorted: Vec<(isize, isize)> = if key.0 == 0 {
                        parts
                            .iter()
                            .sorted_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)))
                            .copied()
                            .collect()
                    } else {
                        parts
                            .iter()
                            .sorted_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)))
                            .copied()
                            .collect()
                    };
                    sorted
                        .iter()
                        .enumerate()
                        .filter(|(i, current)| {
                            let next = sorted.get(i + 1);
                            if key.0 == 0 {
                                next.is_none()
                                    || !(current.1 == next.unwrap().1
                                        && current.0 + 1 == next.unwrap().0)
                            } else {
                                next.is_none()
                                    || !(current.0 == next.unwrap().0
                                        && current.1 + 1 == next.unwrap().1)
                            }
                        })
                        .count()
                })
                .sum::<usize>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
