use std::cmp::Ordering;

advent_of_code::solution!(2);

fn is_ok(sequence: &[i32]) -> bool {
    sequence
        .windows(2)
        .map(|x| x[0] - x[1])
        .fold(Some(Ordering::Equal), |acc, x| {
            let ord = x.cmp(&0);
            if x.abs() >= 1 && x.abs() <= 3 {
                match acc {
                    Some(Ordering::Equal) => Some(ord),
                    Some(Ordering::Less) => match ord {
                        Ordering::Less => Some(Ordering::Less),
                        _ => None,
                    },
                    Some(Ordering::Greater) => match ord {
                        Ordering::Greater => Some(Ordering::Greater),
                        _ => None,
                    },
                    None => None,
                }
            } else {
                None
            }
        })
        .is_some()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|line| is_ok(&line))
            .filter(|x| *x)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|line| {
                let mut ok = is_ok(&line);
                for i in 0..line.len() {
                    let mut line = line.clone();
                    line.remove(i);
                    ok = ok || is_ok(&line);
                }
                ok
            })
            .filter(|x| *x)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
