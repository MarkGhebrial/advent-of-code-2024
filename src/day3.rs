use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(s: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for result in re.captures_iter(&s) {
        sum += result[1].parse::<i32>().unwrap() * result[2].parse::<i32>().unwrap();
    }

    sum
}


