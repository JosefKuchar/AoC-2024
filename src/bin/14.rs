use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use pathfinding::matrix::directions::S;
use regex::Regex;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"-?\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let mut numbers = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap());
            Robot {
                position: (numbers.next().unwrap(), numbers.next().unwrap()),
                velocity: (numbers.next().unwrap(), numbers.next().unwrap()),
            }
        })
        .collect()
}

pub fn solve_part_one(input: &str, width: i32, height: i32) -> Option<u32> {
    let mut robots = parse(input);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position.0 += robot.velocity.0;
            robot.position.0 = robot.position.0.rem_euclid(width);
            robot.position.1 += robot.velocity.1;
            robot.position.1 = robot.position.1.rem_euclid(height);
        }
    }

    let mut quadrants: HashMap<(Ordering, Ordering), usize> = HashMap::new();
    for robot in robots.iter() {
        let (x, y) = robot.position;
        let (xo, yo) = (x.cmp(&(width / 2)), y.cmp(&(height / 2)));
        if xo == Ordering::Equal || yo == Ordering::Equal {
            continue;
        }
        quadrants
            .entry((xo, yo))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    Some(quadrants.iter().fold(1, |acc, (_, v)| acc * v) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_part_one(input, 101, 103)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = 101;
    let height = 103;
    let mut seconds = 0;
    let mut robots = parse(input);

    let mut symetric_max_count = 0;
    for i in 0..10000 {
        for robot in robots.iter_mut() {
            robot.position.0 += robot.velocity.0;
            robot.position.0 = robot.position.0.rem_euclid(width);
            robot.position.1 += robot.velocity.1;
            robot.position.1 = robot.position.1.rem_euclid(height);
        }

        let mut found = false;
        let mut robot_positions: HashSet<(i32, i32)> = HashSet::new();
        for robot in robots.iter() {
            if robot_positions.contains(&robot.position) {
                found = true;
                break;
            }
            robot_positions.insert(robot.position);
        }
        if found {
            continue;
        }
        seconds = i;

        for y in 0..height {
            for x in 0..width {
                let mut found = false;
                for robot in robots.iter() {
                    if robot.position == (x, y) {
                        found = true;
                        break;
                    }
                }
                print!("{}", if found { '#' } else { '.' });
            }
            println!();
        }
        println!("Seconds: {}", i + 1);
    }

    Some(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }
}
