use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res = input
        .lines()
        .fold(0,|acc,line| {
            acc + re.captures_iter(line)
                .fold(0, |acc, cap|
                {
                    acc + cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
                })
        });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_digit = Regex::new(r"(\d+),(\d+)").unwrap();
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|don\'t\(\)|do\(\))").unwrap();
    let mut skip = false;
    let res = re.captures_iter(input).fold(0, |mut acc,capture| {
        if capture.get(1).unwrap().as_str() == "don't()"{
            skip = true;
        } else if capture.get(1).unwrap().as_str() == "do()"{
            skip = false;
        }
        if !skip {
            acc += re_digit.captures_iter(capture.get(1).unwrap().as_str())
            .fold(0 , |acc, cap|{
                acc + cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
                });
        }
        acc
    });
    Some(res)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
