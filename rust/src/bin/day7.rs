use std::io::BufRead;
use std::time::Instant;

use aoc_2021::shared;

fn main() {
    let start = Instant::now();
    let carbs = shared::get_file_reader("../inputs/day7.txt").unwrap()
        .lines()
        .next().unwrap().unwrap()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let part_1_solution = part_1(&carbs);
    println!("Part 1: {}", part_1_solution);
    let part_2_solution = part_2(&carbs);
    println!("Part 2: {}", part_2_solution);
    shared::print_elapsed_time(start)
}

fn part_1(crabs: &Vec<i64>) -> i64 {
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();
    let mut result = i64::MAX;

    (min..=max).for_each(|position| {
        let mut new_result = 0;
        crabs.iter().for_each(|c| new_result += (c - position).abs());
        result = result.min(new_result);
    });
    result
}

fn part_2(crabs: &Vec<i64>) -> i64 {
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();
    let mut result = i64::MAX;

    (min..=max).for_each(|position| {
        let mut new_result = 0;
        crabs.iter().for_each(|c| {
            let diff = (c - position).abs();
            new_result += (diff.pow(2) + diff)/2;
        });
        result = result.min(new_result);
    });
    result
}

#[cfg(test)]
mod day7_tests {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input()), 37);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input()), 168);
    }

    fn get_input() -> Vec<i64> {
        "16,1,2,0,4,2,7,1,2,14".to_string()
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    }
}