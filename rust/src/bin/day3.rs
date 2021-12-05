use std::time::Instant;

use aoc_2021::shared;

fn main() {
    let start = Instant::now();
    let args = shared::get_args();
    let mut reader = shared::get_csv_reader(args[0].as_str(), false, b' ').unwrap();
    let input: Vec<Vec<bool>> = reader.records()
        .map(|rec| rec.unwrap().as_slice().chars().collect::<Vec<char>>()
            .iter().map(|c| *c == '1').collect::<Vec<bool>>())
        .collect();
    let part_1_solution = part_1(&input);
    println!("Part 1: {}", part_1_solution);
    let part_2_solution = part_2(&input);
    println!("Part 2: {}", part_2_solution);
    shared::print_elapsed_time(start);
}

fn part_1(input: &Vec<Vec<bool>>) -> i32 {
    let num_cols: usize = input[0].len();
    let mut gamma_rate = vec![0; num_cols];
    input.iter().for_each(|row| {
        let mut col: usize = 0;
        row.iter().for_each(|bit| {
            match bit {
                true => gamma_rate[col] += 1,
                false => gamma_rate[col] -= 1,
            }
            col += 1;
        });
    });
    gamma_rate = gamma_rate.iter().map(|rate| {
        if *rate > 0 {
            1
        } else {
            0
        }
    }).collect::<Vec<i32>>();
    let epsilon_rate = gamma_rate.iter().map(|rate| {
        if *rate > 0 {
            0
        } else {
            1
        }
    }).collect::<Vec<i32>>();
    binary_to_decimal(gamma_rate) * binary_to_decimal(epsilon_rate)
}

fn part_2(input: &Vec<Vec<bool>>) -> i32 {
    let o2_rating = get_rating(input.clone(), true);
    let co2_rate = get_rating(input.clone(), false);
    o2_rating * co2_rate
}

fn get_rating(mut input: Vec<Vec<bool>>, bit_type: bool) -> i32 {
    let mut binary_bool: Vec<bool> = vec![];
    let num_cols: usize = input[0].len();
    let mut most_common = 0;
    for col in 0..num_cols {
        let input_length = input.len();
        if input_length == 1 {
            binary_bool = input.first().unwrap().to_vec();
            break;
        }
        if input_length == 2 {
            binary_bool = input.iter().find(|row| row[col] == bit_type).unwrap().to_vec();
            break;
        }
        input.iter().for_each(|row| {
            match row[col] {
                true => most_common += 1,
                false => most_common -= 1,
            }
        });
        let most_common_bit;
        match bit_type {
            true => if most_common > 0 { most_common_bit = true } else if most_common < 0 { most_common_bit = false } else { most_common_bit = true },
            false => if most_common > 0 { most_common_bit = false } else if most_common < 0 { most_common_bit = true } else { most_common_bit = false },
        }
        most_common = 0;
        input = input.iter().filter(|row| row[col] == most_common_bit).cloned().collect();
    }
    let binary = binary_bool.iter().map(|bit| if *bit { 1 } else { 0 }).collect::<Vec<i32>>();
    binary_to_decimal(binary)
}

fn binary_to_decimal(binary: Vec<i32>) -> i32 {
    let mut decimal: i32 = 0;
    let mut power: u32 = 0;
    for bit in binary.iter().rev() {
        decimal += bit * 2i32.pow(power);
        power += 1;
    }
    decimal
}

#[cfg(test)]
mod day3_test {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        let actual = part_1(&get_input());
        assert_eq!(actual, 198);
    }

    #[test]
    fn test_part_2() {
        let actual = part_2(&get_input());
        assert_eq!(actual, 230);
    }

    fn get_input() -> Vec<Vec<bool>> {
        vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ].iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .map(|b| b.iter().map(|c| *c == '1').collect::<Vec<bool>>())
            .collect()
    }
}