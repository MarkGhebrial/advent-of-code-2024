use aoc_runner_derive::{aoc, aoc_generator};

use crate::util;

#[aoc_generator(day1)]
pub fn parse_input(s: &str) -> (Vec::<i32>, Vec::<i32>) {
    let mut out = (vec![], vec![]);

    for line in s.split("\n") {
        let mut iterator = line.trim().split("   ").map(|s| {println!("{}", s); s.parse::<i32>().unwrap()});

        out.0.push(iterator.next().unwrap());
        out.1.push(iterator.next().unwrap());
    }

    return out;
}

#[aoc(day1, part1)]
pub fn day1_part1(input: &(Vec::<i32>, Vec::<i32>)) -> i32 {
    let mut sum = 0;

    println!("HELLO WORLD");

    let list_a = util::merge_sort(&input.0);
    let list_b = util::merge_sort(&input.1);

    for i in 0..list_a.len() {
        println!("{}   {}", list_a[i], list_b[i]);

        sum += (list_a[i] - list_b[i]).abs();
    }
    
    sum
}