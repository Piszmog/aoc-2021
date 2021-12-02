use std::str::FromStr;
use std::time::Instant;

use aoc_2021::shared;

fn main() {
    let start = Instant::now();
    let args = shared::get_args();
    let file_path = args[0].as_str();

    let mut records: Vec<i32> = Vec::new();
    let mut reader = shared::get_csv_reader(file_path, false, b',').unwrap();
    for result in reader.records() {
        let record = result.unwrap();
        records.push(FromStr::from_str(&record[0]).unwrap());
    }

    let part_1_solution = part_1(&records);
    println!("Part 1: {}", part_1_solution);
    let part_2_solution = part_2(&records);
    println!("Part 2: {}", part_2_solution);
    shared::print_elapsed_time(start);
}

fn part_1(records: &Vec<i32>) -> i32 {
    let mut increases = 0;
    let mut previous_depth = &i32::MAX;
    for depth in records {
        if depth > previous_depth {
            increases += 1;
        }
        previous_depth = depth;
    }

    increases
}

fn part_2(records: &Vec<i32>) -> i32 {
    let mut sum_measurements: Vec<i32> = Vec::new();
    let mut previous_sum = i32::MAX;
    let mut increases = 0;
    for depth in records {
        if sum_measurements.iter().sum::<i32>() > previous_sum {
            increases += 1;
            previous_sum = sum_measurements.iter().sum::<i32>();
        }
        if sum_measurements.len() == 3 {
            previous_sum = sum_measurements.iter().sum::<i32>();
            sum_measurements.remove(0);
        }
        sum_measurements.push(*depth);
    }
    if sum_measurements.iter().sum::<i32>() > previous_sum {
        increases += 1;
    }

    increases
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        let inputs = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let actual = part_1(&inputs);
        assert_eq!(7, actual);
    }

    #[test]
    fn test_part_2() {
        let inputs = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let actual = part_2(&inputs);
        assert_eq!(5, actual);
    }
}