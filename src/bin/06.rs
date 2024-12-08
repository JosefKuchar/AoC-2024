use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(6);

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(map: &[Vec<char>], pos: (isize, isize)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as isize && pos.1 < map[0].len() as isize
}

fn check_map(map: &[Vec<char>], start: (isize, isize)) -> (bool, Vec<(isize, isize)>) {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut visited_dir: HashSet<((isize, isize), usize)> = HashSet::new();
    let mut dir = 0;
    let mut current = start;
    loop {
        if visited_dir.contains(&(current, dir)) {
            return (true, Vec::new());
        }
        visited.insert(current);
        visited_dir.insert((current, dir));
        let next = (current.0 + DIRS[dir].0, current.1 + DIRS[dir].1);
        if !in_bounds(map, next) {
            return (false, visited.iter().copied().collect());
        }
        if map[next.0 as usize][next.1 as usize] == '#' {
            dir = (dir + 1) % 4;
            continue;
        }
        current = next;
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start: (isize, isize) = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == '^' {
                start = (i as isize, j as isize);
            }
        }
    }
    (map, start)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start) = parse_input(input);
    let (_, visited) = check_map(&map, start);
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start) = parse_input(input);
    let (_, visited) = check_map(&map, start);
    Some(
        visited
            .par_iter()
            .filter(|pos| {
                if (pos.0 == start.0 && pos.1 == start.1)
                    || map[pos.0 as usize][pos.1 as usize] == '#'
                {
                    return false;
                }
                let mut map = map.clone();
                map[pos.0 as usize][pos.1 as usize] = '#';
                let (looped, _) = check_map(&map, start);
                looped
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
