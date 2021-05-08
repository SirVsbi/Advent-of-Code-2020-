use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;
use std::io;
use std::path::Path;

fn part1(numbers: &Vec<i64>, sum: i64) -> Option<(i64, i64)> {
    for number in numbers {
        let missing = sum - *number;
        if numbers.contains(&missing) {
            return Some((*number, missing));
        }
    }
    None
}

fn part2(numbers: &Vec<i64>) -> Option<(i64, i64, i64)> {
    for number in numbers {
        let missing = 2000 - *number;
        if let Some((a, b)) = part1(numbers, missing) {
            if a != b && a != missing {
                return Some((a, b, *number));
            }
        }
    }
    None
}

fn parse_input() -> Vec<i64> {
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    numbers
}

fn main() {
    let numbers = parse_input();
    println!("{:?}", numbers);
    println!("part1: ");
    match part1(&numbers, 2000) {
        Some((a, b)) => {
            println!("Result: {} * {} == {}", a, b, a * b);
        }
        None => println!("solution not found"),
    }

    println!("part2: ");
    match part2(&numbers) {
        Some((a, b, c)) => {
            println!("Result: {} * {} * {} == {}", a, b, c, a * b);
        }
        None => println!("solution not found"),
    }
}

