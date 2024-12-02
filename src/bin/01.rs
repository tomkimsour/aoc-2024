use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_col: Vec<u32> = Vec::new();
    let mut right_col: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
        let (c1, c2) = line.split_once("   ").unwrap();
        left_col.push(c1.parse::<u32>().unwrap());
        right_col.push(c2.parse::<u32>().unwrap());
    });

    left_col.sort_unstable();
    right_col.sort_unstable();

    Some(
        left_col
            .into_iter()
            .zip(right_col)
            .map(|(a, b)| a.abs_diff(b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_col: Vec<u32> = Vec::new();
    let mut right_col: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|line| {
        let (c1, c2) = line.split_once("   ").unwrap();
        left_col.push(c1.parse::<u32>().unwrap());
        let c2_u32 = c2.parse::<u32>().unwrap();
        match right_col.get(&c2_u32) {
            Some(&number) => right_col.insert(c2_u32, number + 1),
            _ => right_col.insert(c2_u32, 1),
        };
    });

    Some(
        left_col
            .into_iter()
            .map(|val| match right_col.get(&val) {
                Some(&number) => val * number,
                _ => 0,
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
