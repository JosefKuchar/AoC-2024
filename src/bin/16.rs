advent_of_code::solution!(16);

use itertools::Itertools;
use pathfinding::prelude::{astar, astar_bag};

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(map: &[Vec<char>], pos: (i32, i32)) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && pos.0 < map.len() as i32
        && pos.1 < map[0].len() as i32
        && map[pos.0 as usize][pos.1 as usize] != '#'
}

fn successors(map: &[Vec<char>], node: &(i32, i32, i32)) -> Vec<((i32, i32, i32), u32)> {
    let mut result = Vec::new();
    let dir = node.2 as usize;
    let straight = (node.0 + DIRS[dir].0, node.1 + DIRS[dir].1);
    if in_bounds(&map, straight) {
        result.push(((straight.0, straight.1, node.2), 1));
    }
    let left = (
        node.0 + DIRS[(dir + 3) % 4].0,
        node.1 + DIRS[(dir + 3) % 4].1,
    );
    if in_bounds(&map, left) {
        result.push(((left.0, left.1, (node.2 + 3) % 4), 1001));
    }
    let right = (
        node.0 + DIRS[(dir + 1) % 4].0,
        node.1 + DIRS[(dir + 1) % 4].1,
    );
    if in_bounds(&map, right) {
        result.push(((right.0, right.1, (node.2 + 1) % 4), 1001));
    }
    result
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32, i32)) {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut start: (i32, i32, i32) = (0, 0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (y as i32, x as i32, 1);
            }
        }
    }
    (map, start)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start) = parse_input(input);
    let res = astar(
        &start,
        |node| successors(&map, node),
        |_| 0,
        |node| map[node.0 as usize][node.1 as usize] == 'E',
    );
    Some(res.unwrap().1 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start) = parse_input(input);
    let res = astar_bag(
        &start,
        |node| successors(&map, node),
        |_| 0,
        |node| map[node.0 as usize][node.1 as usize] == 'E',
    );
    Some(
        res.unwrap()
            .0
            .flat_map(|nodes| {
                nodes
                    .iter()
                    .map(|node| (node.0, node.1))
                    .collect::<Vec<_>>()
            })
            .unique()
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
