use std::collections::HashMap;
use std::io::BufRead;
use std::time::Instant;

use aoc_2021::shared;

fn main() {
    let start = Instant::now();
    let args = shared::get_args();
    let lines = shared::get_file_reader(args.get(0).unwrap()).unwrap()
        .lines()
        .map(|s| {
            let string = s.unwrap();
            let points: Vec<&str> = string.split("->").map(|s| s.trim()).collect();
            let start: Vec<i32> = points[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            let end: Vec<i32> = points[1].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            Line {
                start: Point { x: start[0], y: start[1] },
                end: Point { x: end[0], y: end[1] },
            }
        }).collect::<Vec<Line>>();

    let part_1_solution = get_num_overlap(&lines, false);
    println!("Part 1: {}", part_1_solution);
    let part_2_solution = get_num_overlap(&lines, true);
    println!("Part 2: {}", part_2_solution);
    shared::print_elapsed_time(start);
}

fn get_num_overlap(input: &Vec<Line>, diagonal: bool) -> i32 {
    let mut map: HashMap<Point, i32> = HashMap::new();
    for line in input {
        if !diagonal && line.is_diagonal() {
            continue;
        }
        line.get_path().iter().for_each(|point| {
            if map.contains_key(point) {
                map.insert(point.clone(), map.get(point).unwrap() + 1);
            } else {
                map.insert(point.clone(), 1);
            }
        });
    }
    map.iter().filter(|(_, v)| **v >= 2).count() as i32
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_path(&self) -> Vec<Point> {
        let mut path = Vec::new();
        let mut current = self.start.clone();
        loop {
            path.push(current.clone());
            if current == self.end {
                break;
            }
            if current.x > self.end.x {
                current.x -= 1;
            } else if current.x < self.end.x {
                current.x += 1;
            }
            if current.y > self.end.y {
                current.y -= 1;
            } else if current.y < self.end.y {
                current.y += 1;
            }
        }
        path
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod day5_tests {
    use crate::{get_num_overlap, Line, Point};

    #[test]
    fn test_part_1() {
        let input = get_input();
        assert_eq!(get_num_overlap(&input, false), 5);
    }

    #[test]
    fn test_part_2() {
        let input = get_input();
        assert_eq!(get_num_overlap(&input, true), 12);
    }

    fn get_input() -> Vec<Line> {
        vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
            .iter()
            .map(|s| {
                let points: Vec<&str> = s.split("->").map(|s| s.trim()).collect();
                let start: Vec<i32> = points[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
                let end: Vec<i32> = points[1].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
                Line {
                    start: Point { x: start[0], y: start[1] },
                    end: Point { x: end[0], y: end[1] },
                }
            }).collect()
    }
}