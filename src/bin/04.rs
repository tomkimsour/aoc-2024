advent_of_code::solution!(4);
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
    word: Vec<u8>,
    row_len: usize,
    col_len: usize,
    row_valid_mas: HashMap<(usize, usize), u8>,
    diag_valid_mas: HashMap<(usize, usize), u8>,
}

impl Grid {
    fn new(input: &str, word: Vec<u8>) -> Self {
        let grid = input
            .lines()
            .map(|line| line.bytes().collect_vec())
            .collect_vec();
        let row_len = grid.len();
        let col_len = grid[0].len();
        let row_valid_mas: HashMap<(usize, usize), u8> = HashMap::new();
        let diag_valid_mas: HashMap<(usize, usize), u8> = HashMap::new();
        Self {
            grid,
            word,
            row_len,
            col_len,
            row_valid_mas,
            diag_valid_mas,
        }
    }
    fn solve(&mut self) -> u32 {
        let mut counter: u32 = 0;
        for i in 0..self.row_len {
            for j in 0..self.col_len {
                counter += self.find_letters(i, j, 0);
            }
        }
        counter
    }
    fn get_number_of_valid_mas(&self) -> u32 {
        for (key, val) in self.diag_valid_mas.iter() {
            println!("diag vals ({},{}) : {}", key.0, key.1, val);
        }
        let valid_diag: u32 = self
            .diag_valid_mas
            .values()
            .map(|&v| if v >= 2 { 1 } else { 0 })
            .sum();
        valid_diag
    }

    fn check_down(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        if word_index + 1 >= self.word.len() {
            return true;
        }
        self.check_down(row_index + 1, col_index, word_index + 1)
    }

    fn check_up(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        if word_index + 1 >= self.word.len() {
            return true;
        }
        self.check_up(row_index - 1, col_index, word_index + 1)
    }

    fn check_right(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        if word_index + 1 >= self.word.len() {
            return true;
        }
        self.check_right(row_index, col_index + 1, word_index + 1)
    }

    fn check_left(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        if word_index + 1 >= self.word.len() {
            return true;
        }
        self.check_left(row_index, col_index - 1, word_index + 1)
    }

    fn check_up_left(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        if word_index + 1 >= self.word.len() {
            return true;
        }
        self.check_up_left(row_index - 1, col_index - 1, word_index + 1)
    }

    fn check_up_right(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        if word_index + 1 >= self.word.len() {
            return true;
        }
        self.check_up_right(row_index - 1, col_index + 1, word_index + 1)
    }

    fn check_down_left(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        if word_index + 1 >= self.word.len() {
            return true;
        }
        self.check_down_left(row_index + 1, col_index - 1, word_index + 1)
    }

    fn check_down_right(&self, row_index: usize, col_index: usize, word_index: usize) -> bool {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return false;
        }
        if word_index + 1 >= self.word.len() {
            return true;
        }
        self.check_down_right(row_index + 1, col_index + 1, word_index + 1)
    }

    fn find_letters(&mut self, row_index: usize, col_index: usize, word_index: usize) -> u32 {
        if self.grid[row_index][col_index] != self.word[word_index] {
            return 0;
        }
        println!("---------");
        let mut valid_answers = 0;
        if row_index + self.word.len() <= self.row_len {
            if self.check_down(row_index + 1, col_index, word_index + 1) {
                println!("valid down");
                self.row_valid_mas
                    .entry((row_index + 1, col_index))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                valid_answers += 1;
            }
        }
        if self.word.len() - 1 <= row_index {
            if self.check_up(row_index - 1, col_index, word_index + 1) {
                println!("valid up");
                self.row_valid_mas
                    .entry((row_index - 1, col_index))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                valid_answers += 1;
            }
        }
        if col_index + self.word.len() <= self.col_len {
            if self.check_right(row_index, col_index + 1, word_index + 1) {
                println!("valid right");
                self.row_valid_mas
                    .entry((row_index, col_index + 1))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                valid_answers += 1;
            }
        }
        if self.word.len() - 1 <= col_index {
            if self.check_left(row_index, col_index - 1, word_index + 1) {
                println!("valid left");
                self.row_valid_mas
                    .entry((row_index, col_index - 1))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                valid_answers += 1;
            }
        }

        // check up left, up right, down left, down, right
        if self.word.len() - 1 <= row_index && self.word.len() - 1 <= col_index {
            if self.check_up_left(row_index - 1, col_index - 1, word_index + 1) {
                println!("valid up left");
                self.diag_valid_mas
                    .entry((row_index - 1, col_index - 1))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                valid_answers += 1;
            }
        }
        if self.word.len() - 1 <= row_index && col_index + self.word.len() <= self.col_len {
            if self.check_up_right(row_index - 1, col_index + 1, word_index + 1) {
                println!("valid up right");
                self.diag_valid_mas
                    .entry((row_index - 1, col_index + 1))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                valid_answers += 1;
            }
        }
        if row_index + self.word.len() <= self.row_len && self.word.len() - 1 <= col_index {
            if self.check_down_left(row_index + 1, col_index - 1, word_index + 1) {
                println!("valid down left");
                self.diag_valid_mas
                    .entry((row_index + 1, col_index - 1))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                valid_answers += 1;
            }
        }
        if row_index + self.word.len() <= self.row_len
            && col_index + self.word.len() <= self.col_len
        {
            if self.check_down_right(row_index + 1, col_index + 1, word_index + 1) {
                println!("valid down right");
                self.diag_valid_mas
                    .entry((row_index + 1, col_index + 1))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                valid_answers += 1;
            }
        }

        if valid_answers > 1 {
            println!("Found multiple answers {}", valid_answers);
            println!("{} {}", row_index, col_index);
        } else if valid_answers == 1 {
            println!("Found one solution at {} {}", row_index, col_index);
        }
        valid_answers
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let word = vec![b'X', b'M', b'A', b'S'];
    let mut sgrid = Grid::new(input, word);
    Some(sgrid.solve())
}

pub fn part_two(input: &str) -> Option<u32> {
    let word = vec![b'M', b'A', b'S'];
    let mut sgrid = Grid::new(input, word);
    sgrid.solve();
    Some(sgrid.get_number_of_valid_mas())
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
