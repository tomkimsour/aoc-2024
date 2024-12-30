advent_of_code::solution!(4);
use itertools::Itertools;

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

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
    word: Vec<u8>,
    row_len: usize,
    col_len: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.bytes().collect_vec())
            .collect_vec();
        let word = vec![b'X', b'M', b'A', b'S'];
        let row_len = grid.len();
        let col_len = grid[0].len();
        Self {
            grid,
            word,
            row_len,
            col_len,
        }
    }
    fn solve(&self) -> u32 {
        let mut counter: u32 = 0;
        for i in 0..self.row_len {
            for j in 0..self.col_len {
                counter += self.find_letters(i, j, 0);
            }
        }
        println!("{}", counter);
        counter
    }

    fn find_letter(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if word_index == self.word.len() {
            return true;
        }
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        let mut valid_answer = false;
        // Check up right, left, down, up
        if row_index < self.row_len - 1 {
            valid_answer =
                valid_answer || self.find_letter(row_index + 1, col_index, word_index + 1);
        }
        if 0 < row_index {
            valid_answer =
                valid_answer || self.find_letter(row_index - 1, col_index, word_index + 1);
        }
        if col_index < self.col_len - 1 {
            valid_answer =
                valid_answer || self.find_letter(row_index, col_index + 1, word_index + 1);
        }
        if 0 < col_index {
            valid_answer =
                valid_answer || self.find_letter(row_index, col_index - 1, word_index + 1);
        }

        // check up left, up right, down left, down, right
        if 0 < row_index && 0 < col_index {
            valid_answer =
                valid_answer || self.find_letter(row_index - 1, col_index - 1, word_index + 1);
        }
        if 0 < row_index && col_index < self.col_len - 1 {
            valid_answer =
                valid_answer || self.find_letter(row_index - 1, col_index + 1, word_index + 1);
        }
        if row_index < self.row_len - 1 && 0 < col_index {
            valid_answer =
                valid_answer || self.find_letter(row_index + 1, col_index - 1, word_index + 1);
        }
        if row_index < self.row_len - 1 && col_index < self.col_len - 1 {
            valid_answer =
                valid_answer || self.find_letter(row_index + 1, col_index + 1, word_index + 1);
        }
        valid_answer
    }

    fn find_letters(&self, row_index: usize, col_index: usize, word_index: usize) -> u32 {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return 0;
        }
        let mut valid_answers = 0;
        if row_index < self.row_len - self.word.len()  {
            if self.find_letter(row_index + 1, col_index, word_index + 1) {
                valid_answers += 1;
            }
        }
        if 0 < row_index {
            if self.find_letter(row_index - 1, col_index, word_index + 1) {
                valid_answers += 1;
            }
        }
        if col_index < self.col_len - 1 {
            if self.find_letter(row_index, col_index + 1, word_index + 1) {
                valid_answers += 1;
            }
        }
        if 0 < col_index {
            if self.find_letter(row_index, col_index - 1, word_index + 1) {
                valid_answers += 1;
            }
        }

        // check up left, up right, down left, down, right
        if 0 < row_index && 0 < col_index {
            if self.find_letter(row_index - 1, col_index - 1, word_index + 1) {
                valid_answers += 1;
            }
        }
        if 0 < row_index && col_index < self.col_len - 1 {
            if self.find_letter(row_index - 1, col_index + 1, word_index + 1) {
                valid_answers += 1;
            }
        }
        if row_index < self.row_len - 1 && 0 < col_index {
            if self.find_letter(row_index + 1, col_index - 1, word_index + 1) {
                valid_answers += 1;
            }
        }
        if row_index < self.row_len - 1 && col_index < self.col_len - 1 {
            if self.find_letter(row_index + 1, col_index + 1, word_index + 1) {
                valid_answers += 1;
            }
        }
        if valid_answers > 1 {
            println!("Found multiple answers {}", valid_answers);
            println!("{} {}", row_index, col_index);
        }
        valid_answers
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sgrid = Grid::new(input);
    Some(sgrid.solve())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
