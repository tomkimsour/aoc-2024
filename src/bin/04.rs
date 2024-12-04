advent_of_code::solution!(4);

// 1 1
// 1 1

// 1 1 1 1

// 1
// 1
// 1
// 1

// 1 1 1    1 1 1    1 0 0    0 0 1
// 0 0 1    1 0 0    1 1 1    1 1 1

// 1 1   1 1   1 0   0 1
// 0 1   1 0   1 0   0 1
// 0 1   1 0   1 1   1 1

// 0 1 0    1 1 1
// 1 1 1    0 1 0

// 0 1    1 0
// 1 1    1 1
// 0 1    1 0

// 1 0    0 1
// 1 1    1 1
// 0 1    1 0

// 0 1 1   1 1 0
// 1 1 0   0 1 1

// 1 0 0 0    0 0 0 1
// 0 1 0 0    0 0 1 0
// 0 0 1 0    0 1 0 0
// 0 0 0 1    1 0 0 0

// 1 0 0    1 0 0    1 0 0    1 1 0
// 0 1 1    0 1 0    1 1 0    0 1 0
// 0 0 1    0 1 1    0 0 1    0 0 1

// 0 0 0    0 0 1    1 0 1    1 1 0
// 0 1 1    0 1 0    1 1 0    0 1 0
// 1 0 1    0 1 1    0 0 0    1 0 0

pub fn part_one(input: &str) -> Option<u32> {
    // it s an extended version of tetris
    // you need to count the amount of tetris pieces
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
