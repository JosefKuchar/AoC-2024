advent_of_code::solution!(10);

use pathfinding::prelude::dijkstra_all;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn in_bounds(map: &[Vec<i32>], pos: (isize, isize)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as isize && pos.1 < map[0].len() as isize
}

pub fn successors(map: &[Vec<i32>], pos: (usize, usize)) -> Vec<((usize, usize), u32)> {
    let mut res = Vec::new();
    let val = map[pos.0][pos.1];
    for dir in &DIRS {
        let next = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
        if in_bounds(map, next) {
            let next_val = map[next.0 as usize][next.1 as usize];
            if next_val - 1 == val {
                res.push(((next.0 as usize, next.1 as usize), 1));
            }
        }
    }
    res
}

fn solve(input: &str, func: fn(map: &[Vec<i32>], pos: (usize, usize)) -> u32) -> Option<u32> {
    let map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                sum += func(&map, (i, j));
            }
        }
    }
    Some(sum)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, |map, pos| {
        dijkstra_all(&pos, |&pos| successors(map, pos))
            .iter()
            .filter(|((i, j), _)| map[*i][*j] == 9)
            .count() as u32
    })
}

fn find_recursive(map: &[Vec<i32>], pos: (usize, usize)) -> u32 {
    if map[pos.0][pos.1] == 9 {
        return 1;
    }
    let next = successors(map, pos);
    next.iter().map(|(pos, _)| find_recursive(map, *pos)).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, find_recursive)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
