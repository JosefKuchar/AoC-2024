use regex::Regex;
advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

pub fn solve(input: &str, offset: isize) -> Option<u64> {
    let re = Regex::new(r"\d+").unwrap();
    Some(
        input
            .split("\n\n")
            .map(|machine| {
                let numbers: Vec<Vec<isize>> = machine
                    .lines()
                    .map(|line| {
                        re.captures_iter(line)
                            .map(|cap| cap[0].parse::<isize>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect();
                Machine {
                    a: (numbers[0][0], numbers[0][1]),
                    b: (numbers[1][0], numbers[1][1]),
                    prize: (numbers[2][0] + offset, numbers[2][1] + offset),
                }
            })
            .map(|machine| {
                let (a1, b1, c1) = (machine.a.0, machine.b.0, machine.prize.0);
                let (a2, b2, c2) = (machine.a.1, machine.b.1, machine.prize.1);
                let (b3, c3) = (a2 * b1, a2 * c1);
                let (b4, c4) = (a1 * b2, a1 * c2);
                let (mut b5, mut c5) = (b4 - b3, c4 - c3);
                if b5 < 0 && c5 < 0 {
                    b5 = -b5;
                    c5 = -c5;
                }
                if b5 < 0 || c5 < 0 {
                    return 0;
                }
                if c5 % b5 != 0 {
                    return 0;
                }
                let b6 = c5 / b5;
                let b7 = c1 - b1 * b6;
                if b7 < 0 {
                    return 0;
                }
                if b7 % a1 != 0 {
                    return 0;
                }
                let a6 = b7 / a1;
                a6 * 3 + b6
            })
            .sum::<isize>() as u64,
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 0)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
