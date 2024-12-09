use std::iter::repeat;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum Location {
    Data(u32, u32),
    Space(u32),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk: Vec<Option<u64>> = Vec::new();
    let mut space = false;
    let mut block_num = 0;
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .for_each(|c| {
            if space {
                disk.extend(repeat(None).take(c as usize));
            } else {
                disk.extend(repeat(Some(block_num)).take(c as usize));
                block_num += 1;
            }
            space = !space;
        });
    let mut ptr1 = 0;
    let mut ptr2 = disk.len() - 1;
    while ptr1 < ptr2 {
        if disk[ptr1].is_some() {
            ptr1 += 1;
        } else if disk[ptr2].is_none() {
            ptr2 -= 1;
        } else {
            disk[ptr1] = disk[ptr2];
            disk[ptr2] = None;
            ptr1 += 1;
            ptr2 -= 1;
        }
    }
    Some(
        disk.iter()
            .enumerate()
            .map(|(i, x)| if let Some(x) = x { x * i as u64 } else { 0 })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk: Vec<Location> = Vec::new();
    let mut space = false;
    let mut block_num = 0;
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .for_each(|c| {
            if space {
                disk.push(Location::Space(c));
            } else {
                disk.push(Location::Data(c, block_num));
                block_num += 1;
            }
            space = !space;
        });
    let mut to_move = disk.len();
    while to_move > 0 {
        to_move -= 1;
        match disk[to_move] {
            Location::Space(_) => continue,
            Location::Data(data_size, block_num) => {
                let mut location = 0;
                // Move data
                while location < to_move {
                    match disk[location] {
                        Location::Space(space_size) => {
                            if space_size >= data_size {
                                let diff = space_size - data_size;
                                disk[location] = Location::Data(data_size, block_num);
                                if diff > 0 {
                                    disk.insert(location + 1, Location::Space(diff));
                                    to_move += 1;
                                }
                                disk[to_move] = Location::Space(data_size);
                                break;
                            } else {
                                location += 1;
                            }
                        }
                        Location::Data(_, _) => {
                            location += 1;
                        }
                    }
                }
                // Consolidate space
                let mut ptr = 0;
                loop {
                    if ptr + 1 >= disk.len() {
                        break;
                    }
                    match (disk[ptr], disk[ptr + 1]) {
                        (Location::Space(a), Location::Space(b)) => {
                            disk[ptr] = Location::Space(a + b);
                            disk.remove(ptr + 1);
                        }
                        _ => {
                            ptr += 1;
                        }
                    }
                }
            }
        }
    }
    let mut i = 0;
    let mut sum = 0;
    disk.iter().for_each(|loc| match loc {
        Location::Data(size, block_num) => {
            for _ in 0..*size {
                sum += i * *block_num as u64;
                i += 1;
            }
        }
        Location::Space(size) => i += *size as u64,
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
