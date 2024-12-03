use aoc_runner_derive::{aoc, aoc_generator};

use crate::util;

pub fn part1(s: &str) -> i32 {
    let input = parse_input(s);
    day1_part1(&input)
}

pub fn part2(s: &str) -> i32 {
    let input = parse_input(s);
    day1_part2(&input)
}

#[aoc_generator(day1)]
pub fn parse_input(s: &str) -> (Vec<i32>, Vec<i32>) {
    let mut out = (vec![], vec![]);

    for line in s.split("\n") {
        let mut iterator = line.trim().split("   ").map(|s| {
            println!("{}", s);
            s.parse::<i32>().unwrap()
        });

        out.0.push(iterator.next().unwrap());
        out.1.push(iterator.next().unwrap());
    }

    return out;
}

#[aoc(day1, part1)]
pub fn day1_part1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
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

#[aoc(day1, part2)]
pub fn day1_part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut score = 0;

    println!("HELLO WORLD");

    let left_list = util::merge_sort(&input.0);
    let right_list = util::merge_sort(&input.1);

    let mut previous_number: i32 = -1;
    for number in left_list.iter() {
        if *number == previous_number {
            continue;
        } else {
            previous_number = *number;
        }

        // Find the number of occurrences of `number` in the right list
        let mut occurrences = 0;
        let mut number_has_been_found = false;
        for n in right_list.iter() {
            if n == number {
                occurrences += 1;
                number_has_been_found = true;
            } else if number_has_been_found {
                break;
            }
        }

        score += occurrences * number;
    }

    score
}
