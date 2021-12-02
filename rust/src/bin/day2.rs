use std::time::Instant;

use serde::Deserialize;

use aoc_2021::shared;

#[derive(Debug, Deserialize)]
struct Course {
    direction: String,
    value: i32,
}

fn main() {
    let start = Instant::now();
    let mut reader = shared::get_csv_reader("../inputs/day2.txt", false, b' ').unwrap();
    let courses: Vec<Course> = reader.deserialize().map(|result| {
        let course: Course = result.unwrap();
        course
    }).collect();

    let part_1_solution = part_1(&courses);
    println!("Part 1: {}", part_1_solution);
    let part_2_solution = part_2(&courses);
    println!("Part 2: {}", part_2_solution);

    shared::print_elapsed_time(start);
}

fn part_1(input: &Vec<Course>) -> i32 {
    let mut horizontal_sum = 0;
    let mut depth_sum = 0;
    input.iter().for_each(|course| {
        match course.direction.as_str() {
            "forward" => horizontal_sum += course.value,
            "down" => depth_sum += course.value,
            "up" => depth_sum -= course.value,
            _ => {}
        }
    });
    horizontal_sum * depth_sum
}

fn part_2(input: &Vec<Course>) -> i32 {
    let mut horizontal_sum = 0;
    let mut depth_sum = 0;
    let mut aim_sum = 0;
    input.iter().for_each(|course| {
        match course.direction.as_str() {
            "forward" => {
                horizontal_sum += course.value;
                depth_sum += course.value * aim_sum;
            }
            "down" => aim_sum += course.value,
            "up" => aim_sum -= course.value,
            _ => {}
        }
    });
    horizontal_sum * depth_sum
}

#[cfg(test)]
mod day2_tests {
    use std::str::FromStr;

    use crate::{Course, part_1, part_2};

    #[test]
    fn test_part_1() {
        let input = get_input();
        let actual = part_1(&input);
        assert_eq!(150, actual);
    }

    #[test]
    fn test_part_2() {
        let input = get_input();
        let actual = part_2(&input);
        assert_eq!(900, actual);
    }

    fn get_input() -> Vec<Course> {
        let raw_input = vec!(
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        );
        raw_input.iter()
            .map(|s| s.split(" "))
            .map(|mut val| Course {
                direction: val.next().unwrap().to_string(),
                value: FromStr::from_str(val.next().unwrap()).unwrap(),
            })
            .collect()
    }
}