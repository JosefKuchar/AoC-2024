use std::collections::HashSet;

advent_of_code::solution!(8);

fn in_bounds(map: &[Vec<char>], pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as i32 && pos.1 < map[0].len() as i32
}

pub fn solve<R: IntoIterator<Item = i32> + Clone>(input: &str, range: R) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: Vec<((usize, usize), char)> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != '.' {
                antennas.push(((i, j), *cell));
            }
        }
    }
    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    antennas.iter().for_each(|a| {
        antennas.iter().filter(|b| a.1 == b.1).for_each(|b| {
            if a.0 != b.0 {
                let (y1, x1) = a.0;
                let (y2, x2) = b.0;
                let (dx, dy) = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32);
                for k in range.clone() {
                    let (ny1, nx1) = (y1 as i32 - dy * k, x1 as i32 - dx * k);
                    if in_bounds(&map, (ny1, nx1)) {
                        nodes.insert((ny1 as usize, nx1 as usize));
                    } else {
                        break;
                    }
                }
                for k in range.clone() {
                    let (ny2, nx2) = (y2 as i32 + dy * k, x2 as i32 + dx * k);
                    if in_bounds(&map, (ny2, nx2)) {
                        nodes.insert((ny2 as usize, nx2 as usize));
                    } else {
                        break;
                    }
                }
            }
        });
    });
    Some(nodes.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 1..=1)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 0..)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
