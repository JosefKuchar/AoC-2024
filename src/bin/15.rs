advent_of_code::solution!(15);

fn find_empty(map: &[Vec<char>], robot: (i32, i32), dir: (i32, i32)) -> Option<(i32, i32)> {
    let mut robot = robot;
    loop {
        robot.0 += dir.0;
        robot.1 += dir.1;
        if map[robot.0 as usize][robot.1 as usize] == '#' {
            return None;
        }
        if map[robot.0 as usize][robot.1 as usize] == '.' {
            return Some(robot);
        }
    }
}

// If can push, return all the positions that can be pushed
// If can't push, return None
fn push_recursive(
    map: &mut [Vec<char>],
    pos: (i32, i32),
    dir: (i32, i32),
) -> Option<Vec<(i32, i32)>> {
    match map[pos.0 as usize][pos.1 as usize] {
        '#' => return None,
        '.' => return Some(Vec::new()),
        _ => (),
    }
    let mut positions: Vec<(i32, i32)> = Vec::new();
    if dir.0 != 0 {
        // Up or down
        positions.push(pos);
        match map[pos.0 as usize][pos.1 as usize] {
            '[' => positions.push((pos.0 + 0, pos.1 + 1)),
            ']' => positions.push((pos.0 + 0, pos.1 - 1)),
            _ => (),
        }
        let mut recursive_positions: Vec<(i32, i32)> = Vec::new();
        for position in &positions {
            let next = (position.0 + dir.0, position.1 + dir.1);
            let res = push_recursive(map, next, dir);
            if res.is_none() {
                return None;
            }
            recursive_positions.extend(res.unwrap());
        }
        positions.extend(recursive_positions);
    } else {
        // Left or right
        positions.push(pos);
        let mut current = pos;
        loop {
            current = (current.0, current.1 + dir.1);
            match map[current.0 as usize][current.1 as usize] {
                '#' => return None,
                '.' => break,
                _ => (),
            }
            positions.push(current);
        }
    }
    Some(positions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");
    let mut map: Vec<Vec<char>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let instructions = parts
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '<' => Some((0, -1)),
            '>' => Some((0, 1)),
            '^' => Some((-1, 0)),
            'v' => Some((1, 0)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut robot = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                robot = (i as i32, j as i32);
            }
        }
    }
    map[robot.0 as usize][robot.1 as usize] = '.';

    for instruction in instructions {
        let empty = find_empty(&map, robot, instruction);
        if empty.is_none() {
            continue;
        }
        let next = (robot.0 + instruction.0, robot.1 + instruction.1);
        if map[next.0 as usize][next.1 as usize] == '.' {
            robot = next;
        } else if map[next.0 as usize][next.1 as usize] == 'O' {
            robot = next;
            map[next.0 as usize][next.1 as usize] = '.';
            map[empty.unwrap().0 as usize][empty.unwrap().1 as usize] = 'O';
        }
    }

    Some(
        map.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, cell)| match cell {
                        'O' => Some(100 * i + j),
                        _ => None,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");
    let mut map: Vec<Vec<char>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| {
                    match c {
                        '#' => "##",
                        'O' => "[]",
                        '.' => "..",
                        '@' => "@.",
                        _ => panic!(),
                    }
                    .chars()
                })
                .collect()
        })
        .collect();
    let instructions = parts
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '<' => Some((0, -1)),
            '>' => Some((0, 1)),
            '^' => Some((-1, 0)),
            'v' => Some((1, 0)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut robot = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                robot = (i as i32, j as i32);
            }
        }
    }
    map[robot.0 as usize][robot.1 as usize] = '.';
    for instruction in instructions {
        let next = (robot.0 + instruction.0, robot.1 + instruction.1);
        if map[next.0 as usize][next.1 as usize] == '.' {
            robot = next;
        } else {
            let to_push = push_recursive(&mut map, next, instruction);
            match to_push {
                Some(positions) => {
                    robot = next;
                    let mut new_positions = Vec::new();
                    for position in &positions {
                        new_positions.push((
                            position.0 + instruction.0,
                            position.1 + instruction.1,
                            map[position.0 as usize][position.1 as usize],
                        ));
                    }
                    for position in &positions {
                        map[position.0 as usize][position.1 as usize] = '.';
                    }
                    for position in new_positions {
                        map[position.0 as usize][position.1 as usize] = position.2;
                    }
                }
                None => (),
            }
        }
    }

    Some(
        map.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, cell)| match cell {
                        '[' => Some(100 * i + j),
                        _ => None,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
