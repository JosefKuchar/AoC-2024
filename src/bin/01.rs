use itertools::Itertools;

advent_of_code::solution!(1);

fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let numbers: Vec<u32> = input
        .lines()
        .flat_map(|line| line.split_whitespace().map(|x| x.parse::<u32>().unwrap()))
        .collect();
    let left = numbers.iter().step_by(2).map(|x| *x).collect();
    let right = numbers.iter().skip(1).map(|x| *x).step_by(2).collect();

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = get_lists(input);
    Some(
        left.iter()
            .sorted()
            .zip(right.iter().sorted())
            .map(|(l, r)| l.abs_diff(*r))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = get_lists(input);
    Some(
        left.iter()
            .map(|l| l * right.iter().filter(|r| *r == l).count() as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
