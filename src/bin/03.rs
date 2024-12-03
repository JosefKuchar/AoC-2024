advent_of_code::solution!(3);

use regex::Regex;

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .map(|capture| match &capture[0] {
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            _ => {
                let a = capture[1].parse::<u32>().unwrap();
                let b = capture[2].parse::<u32>().unwrap();
                Instruction::Mul(a, b)
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_instructions(input).iter().fold(0, |acc, x| match x {
        Instruction::Mul(a, b) => acc + a * b,
        _ => acc,
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_instructions(input)
            .iter()
            .fold((0, true), |acc: (u32, bool), x| match x {
                Instruction::Do => (acc.0, true),
                Instruction::Dont => (acc.0, false),
                Instruction::Mul(a, b) => {
                    if acc.1 {
                        (acc.0 + a * b, acc.1)
                    } else {
                        (acc.0, acc.1)
                    }
                }
            })
            .0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
