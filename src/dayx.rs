use aoc_runner_derive::{aoc, aoc_generator};

use crate::util;

type GeneratorType = String;

pub fn part1(s: &str) -> i32 {
    let input = parse_input(s);
    dayx_part1(&input)
}

pub fn part2(s: &str) -> i32 {
    let input = parse_input(s);
    dayx_part2(&input)
}

#[aoc_generator(dayx)]
pub fn parse_input(s: &str) -> GeneratorType {
    todo!()
}

#[aoc(dayx, part1)]
pub fn dayx_part1(input: &GeneratorType) -> i32 {
    todo!()
}

#[aoc(dayx, part2)]
pub fn dayx_part2(input: &GeneratorType) -> i32 {
    todo!()
}
