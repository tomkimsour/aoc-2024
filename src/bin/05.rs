advent_of_code::solution!(5);
use std::{collections::HashMap, thread::current};

pub fn parse_input(input: &str) -> (impl Iterator<Item = (u32, u32)> + '_, Vec<Vec<u32>>) {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let or = ordering_rules.lines().map(|line| {
        let (left, right) = line.split_once('|').unwrap();
        (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
    });

    let up = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|val| val.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();
    (or, up)
}

pub fn verify_update_line(update_line: Vec<u32>, reverse_hashmap: HashMap<u32, Vec<u32>>) -> bool {
    update_line
        .iter()
        .enumerate()
        .map(|(index, &page_number)| {
            let current_page = page_number;
            let mut second_update_line_iter = update_line.clone();
            second_update_line_iter.truncate(index);
            second_update_line_iter.iter().all(|value| {
                if reverse_hashmap.contains_key(&current_page) {
                    reverse_hashmap.get(&current_page).unwrap().contains(value)
                } else {
                    false
                }
            })
        })
        .all(|val| val)
}

pub fn find_middle_value<T: Clone>(current_vector: &[T]) -> &T {
    let size = current_vector.len() / 2;
    current_vector.get(size).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering_rule, update) = parse_input(input);
    let mut reverse_hashmap: HashMap<u32, Vec<u32>> = HashMap::new();
    for (left, right) in ordering_rule {
        reverse_hashmap
            .entry(right)
            .and_modify(|all_previous_values| all_previous_values.push(left))
            .or_insert(vec![left]);
    }
    let mut res = 0;
    for update_info in update {
        if verify_update_line(update_info.clone(), reverse_hashmap.clone()) {
            let middle_page = find_middle_value::<u32>(&update_info);
            res += middle_page;
        }
    }
    Some(res)
}

/**
* @brief Push the bubble value to the end of the input vector
**/
pub fn bubble_value<T : Clone>(mut current_vector: Vec<T>, bubble_value_index: usize) -> Vec<T>{
    for index in bubble_value_index..current_vector.len() -2{
        current_vector.swap(index, index+1);
    }
    current_vector
}

pub fn sort_non_valid_array(update_line: Vec<u32>, normal_hashmap: HashMap<u32, Vec<u32>>) -> Vec<u32>{
    update_line.iter().map(|val|
        {
            reverse_hashmap.
        }
    )

}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering_rule, update) = parse_input(input);
    let mut reverse_hashmap: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut normal_hashmap: HashMap<u32, Vec<u32>> = HashMap::new();
    for (left, right) in ordering_rule {
        reverse_hashmap
            .entry(right)
            .and_modify(|all_previous_values| all_previous_values.push(left))
            .or_insert(vec![left]);
        normal_hashmap
            .entry(left)
            .and_modify(|all_previous_values| all_previous_values.push(right))
            .or_insert(vec![right]);
    }
    let mut res = 0;
    for mut update_info in update {
        if !verify_update_line(update_info.clone(), reverse_hashmap.clone()) {
            update_info = sort_non_valid_array(update_info.clone(),normal_hashmap.clone());
            let middle_page = find_middle_value::<u32>(&update_info);
            res += middle_page;
        }
    }
    Some(res)
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
        assert_eq!(result, Some(123));
    }
}
