use std::io::BufRead;
use std::str::FromStr;
use std::time::Instant;

use aoc_2021::shared;

fn main() {
    let start = Instant::now();
    let args = shared::get_args();
    let mut row = 0;
    let mut num_line: String = FromStr::from_str("").unwrap();
    let mut board_lines: Vec<String> = Vec::new();
    shared::get_file_reader(args.get(0).unwrap()).unwrap()
        .lines().for_each(|line| {
        if row == 0 {
            num_line = line.unwrap();
        } else {
            board_lines.push(line.unwrap());
        }
        row += 1;
    });
    let call_nums = get_nums(num_line);
    let boards = get_boards(board_lines);

    let part_1_solution = part_1(&call_nums, boards.clone());
    println!("Part 1: {}", part_1_solution);
    let part_2_solution = part_2(&call_nums, boards.clone());
    println!("Part 2: {}", part_2_solution);
    shared::print_elapsed_time(start);
}

fn get_nums(line: String) -> Vec<i32> {
    line.split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn get_boards(lines: Vec<String>) -> Vec<Board> {
    let mut boards = Vec::new();
    let mut input: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            if input.len() > 0 {
                boards.push(Board::new(input.clone()));
                input.clear();
            }
        } else {
            input.push(line.split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect());
        }
    }
    if input.len() > 0 {
        boards.push(Board::new(input.clone()));
    }
    boards
}

fn part_1(call_nums: &Vec<i32>, mut boards: Vec<Board>) -> i32 {
    let mut final_num = 0;
    let mut unmarked_sum = 0;
    'outer: for num in call_nums {
        final_num = *num;
        for board in &mut boards {
            board.mark_square(final_num);
            if board.is_winner() {
                unmarked_sum = board.get_unmarked_sum();
                break 'outer;
            }
        }
    }
    final_num * unmarked_sum
}

fn part_2(call_nums: &Vec<i32>, mut boards: Vec<Board>) -> i32 {
    let mut final_num = 0;
    let mut unmarked_sum = 0;
    'outer: for num in call_nums {
        final_num = *num;
        for i in 0..boards.len() {
            if boards[i].is_winner() {
                continue;
            }
            boards[i].mark_square(*num);
            if boards[i].is_winner() {
                boards[i].make_winner();
            }
            if boards.iter().all(|b| b.is_winner()) {
                unmarked_sum = boards[i].get_unmarked_sum();
                break 'outer;
            }
        }
    }
    final_num * unmarked_sum
}

#[derive(Debug, Clone)]
struct Board {
    size: usize,
    rows: Vec<Row>,
    winner: bool,
}

#[derive(Debug, Clone)]
struct Row {
    squares: Vec<Square>,
}

#[derive(Debug, Clone)]
struct Square {
    marked: bool,
    value: i32,
}

impl Board {
    fn new(input: Vec<Vec<i32>>) -> Board {
        Board {
            size: input.len(),
            rows: input.iter().map(|row| Row::new(row.to_vec())).collect(),
            winner: false,
        }
    }

    fn mark_square(&mut self, num: i32) {
        for row in &mut self.rows {
            for sq in &mut row.squares {
                if sq.value == num {
                    sq.marked = true;
                    return;
                }
            }
        }
    }

    fn get_unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for row in &self.rows {
            for sq in &row.squares {
                if !sq.marked {
                    sum += sq.value;
                }
            }
        }
        sum
    }

    fn is_winner(&self) -> bool {
        let mut is_winner = false;
        for s in 0..self.size {
            if self.is_column_winner(s) || self.is_row_winner(s) {
                is_winner = true;
                break;
            }
        }
        is_winner
    }

    fn make_winner(&mut self) {
        self.winner = true;
    }

    fn is_row_winner(&self, r: usize) -> bool {
        self.rows[r].squares.iter().all(|sq| sq.marked)
    }

    fn is_column_winner(&self, col: usize) -> bool {
        let mut is_marked = true;
        for row in &self.rows {
            is_marked = is_marked && row.get(col).marked;
            if !is_marked {
                break;
            }
        }
        is_marked
    }
}

impl Row {
    fn new(values: Vec<i32>) -> Row {
        Row {
            squares: values.iter().map(|v| Square::new(*v)).collect(),
        }
    }

    fn get(&self, index: usize) -> &Square {
        &self.squares[index]
    }
}

impl Square {
    fn new(value: i32) -> Square {
        Square {
            marked: false,
            value,
        }
    }
}

#[cfg(test)]
mod day4_tests {
    use crate::{Board, get_boards, get_nums, part_1, part_2};

    #[test]
    fn test_part_1() {
        let nums = get_nums_input();
        let boards = get_boards_input();
        assert_eq!(part_1(&nums, boards), 4512)
    }

    #[test]
    fn test_part_2() {
        let nums = get_nums_input();
        let boards = get_boards_input();
        assert_eq!(part_2(&nums, boards), 1924)
    }

    fn get_nums_input() -> Vec<i32> {
        get_nums("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string())
    }

    fn get_boards_input() -> Vec<Board> {
        get_boards(vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
            "",
        ].iter().map(|s| s.to_string()).collect())
    }
}