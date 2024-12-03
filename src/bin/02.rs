advent_of_code::solution!(2);

fn is_ordered(test_input: Vec<i32>) -> bool {
    test_input.windows(2).all(|w| w[0] <= w[1]) || test_input.windows(2).all(|w| w[0] >= w[1])
}
#[allow(dead_code)]
fn is_valid(mut test_input: Vec<i32>) -> bool {
    if is_ordered(test_input.clone()) {
        test_input.sort_unstable();
        return test_input
            .windows(2)
            .all(|w| (1..4).contains(&(w[1] - w[0])));
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let res: usize = input
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            is_valid(values)
        })
        .filter(|&x| x)
        .count();
    #[allow(clippy::cast_possible_truncation)]
    Some(res as u32)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    let res: usize = input
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            is_valid(values.clone())
                || (0..values.len()).any(|i| {
                    let mut values_copy = values.clone();
                    values_copy.remove(i);
                    is_valid(values_copy)
                })
        })
        .filter(|&x| x)
        .count();
    #[allow(clippy::cast_possible_truncation)]
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_line_input() {
        let mut test_input = vec![1, 2, 7, 8, 9];
        assert!(!is_valid(test_input));
        test_input = vec![7, 6, 4, 2, 1];
        assert!(is_valid(test_input));
        test_input = vec![9, 7, 6, 2, 1];
        assert!(!is_valid(test_input));
        test_input = vec![1, 3, 2, 4, 5];
        assert!(!is_valid(test_input));
        test_input = vec![8, 6, 4, 4, 1];
        assert!(!is_valid(test_input));
        test_input = vec![1, 3, 6, 7, 9];
        assert!(is_valid(test_input));
    }
}
