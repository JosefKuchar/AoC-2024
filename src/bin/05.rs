use itertools::Itertools;

advent_of_code::solution!(5);

fn check_order(ordering: &[(u32, u32)], update: &[u32]) -> bool {
    ordering.iter().all(|order| {
        let first = update.iter().find_position(|x| x == &&order.0);
        let second = update.iter().find_position(|x| x == &&order.1);
        match (first, second) {
            (Some((first, _)), Some((second, _))) => first < second,
            _ => true,
        }
    })
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let ordering = parts[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|x| x.parse::<u32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();
    let updates = parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    (ordering, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering, updates) = parse_input(input);

    Some(
        updates
            .iter()
            .filter(|update| check_order(&ordering, update))
            .map(|update| update[update.len() / 2])
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering, updates) = parse_input(input);

    Some(
        updates
            .iter()
            .filter(|update| !check_order(&ordering, update))
            .map(|update| {
                let mut new_update = Vec::new();
                for num in update {
                    for index in 0..update.len() {
                        let mut candidate = new_update.clone();
                        candidate.insert(index, *num);
                        if check_order(&ordering, &candidate) {
                            new_update = candidate;
                            break;
                        }
                    }
                }
                new_update[new_update.len() / 2]
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
