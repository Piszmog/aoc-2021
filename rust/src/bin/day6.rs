use std::collections::HashMap;
use std::io::BufRead;
use std::time::Instant;

use aoc_2021::shared;

fn main() {
    let start = Instant::now();

    let fish = get_fish(shared::get_file_reader("../inputs/day6.txt").unwrap()
        .lines().next().unwrap().unwrap());

    let part_1_solution = get_fish_pop(fish.clone(), 80);
    println!("Part 1: {}", part_1_solution);
    let part_2_solution = get_fish_pop(fish.clone(), 256);
    println!("Part 2: {}", part_2_solution);

    shared::print_elapsed_time(start);
}

const CYCLES: i32 = 9;

fn get_fish(input: String) -> HashMap<i32, i64> {
    let mut fish: HashMap<i32, i64> = HashMap::new();
    (0..CYCLES).for_each(|i| {
        fish.insert(i, 0);
    });
    input.to_string()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .for_each(|x| {
            fish.entry(x).and_modify(|e| *e += 1);
        });
    fish
}

fn get_fish_pop(mut fish: HashMap<i32, i64>, days: usize) -> i64 {
    (0..days).for_each(|_| {
        let mut end_cycle_fish = 0;
        (0..CYCLES).for_each(|i| {
            let num = fish.get(&i).unwrap().clone();
            if i == 0 {
                end_cycle_fish = num;
            } else {
                fish.entry(i - 1).and_modify(|e| *e = num);
            }
        });
        fish.entry(CYCLES - 1).and_modify(|e| *e = end_cycle_fish);
        fish.entry(CYCLES - 3).and_modify(|e| *e += end_cycle_fish);
    });
    fish.iter().map(|(_, v)| v).sum()
}

#[cfg(test)]
mod day6_test {
    use std::collections::HashMap;

    use crate::{get_fish, get_fish_pop};

    #[test]
    fn test_part_1() {
        let fish = get_fish_input();
        assert_eq!(get_fish_pop(fish, 80), 5934);
    }

    #[test]
    fn test_part_2() {
        let fish = get_fish_input();
        assert_eq!(get_fish_pop(fish, 256), 26984457539);
    }

    fn get_fish_input() -> HashMap<i32, i64> {
        get_fish("3,4,3,1,2".to_string())
    }
}