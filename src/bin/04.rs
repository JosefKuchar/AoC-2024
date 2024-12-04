advent_of_code::solution!(4);

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            for (dx, dy) in DIRS.iter() {
                let mut buffer = String::new();
                for k in 0..4 {
                    let x = i as i32 + dx * k;
                    let y = j as i32 + dy * k;
                    if x < 0 || y < 0 || x >= map.len() as i32 || y >= map[i].len() as i32 {
                        break;
                    }
                    buffer.push(map[x as usize][y as usize]);
                }
                if buffer == "XMAS" {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    for i in 1..(map.len() - 1) {
        for j in 1..(map[i].len() - 1) {
            if map[i][j] != 'A' {
                continue;
            }
            let corners = [
                map[i - 1][j - 1],
                map[i - 1][j + 1],
                map[i + 1][j + 1],
                map[i + 1][j - 1],
            ];

            if corners.iter().filter(|c| **c == 'M').count() == 2
                && corners.iter().filter(|c| **c == 'S').count() == 2
            {
                for k in 0..corners.len() {
                    if corners[k] == 'M' && corners[(k + 1) % 4] == 'M' {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
