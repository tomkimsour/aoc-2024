advent_of_code::solution!(5);
use std::collections::HashMap;
use itertools::Itertools;


pub fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = (u32, u32)> + '_,
    impl Iterator<Item = Iterator<Item = u32>> + '_,
) {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let or = ordering_rules.lines().map(|line| {
        let (left, right) = line.split_once('|').unwrap();
        (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
    });

    let up = updates
        .lines()
        .map(|line| line.split(",").map(|val| val.parse::<u32>().unwrap()));
    (or, up)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering_rule, update) = parse_input(input);
    let forward_map: HashMap<u32, Vec<u32>> = HashMap::new();
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
