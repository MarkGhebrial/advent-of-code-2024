use aoc_runner_derive::{aoc, aoc_generator};

pub fn part1(s: &str) -> i32 {
    let input = parse_input(s);
    day2_part1(&input)
}

pub fn part2(s: &str) -> i32 {
    let input = parse_input(s);
    day2_part2(&input)
}

#[aoc_generator(day2)]
pub fn parse_input(s: &str) -> Vec<Vec<i32>> {
    let mut out = vec![];

    for line in s.split("\n") {
        out.push(
            line.trim()
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        );
    }

    return out;
}

#[aoc(day2, part1)]
pub fn day2_part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for report in input {
        if report_is_safe(&report) {
            sum += 1;
        }
    }

    sum
}

fn report_is_safe(report: &[i32]) -> bool {
    let mut increasing: Option<bool> = None;

    for i in 1..report.len() {
        // Make sure the difference between the data points is within the acceptable range
        let difference = (report[i] - report[i - 1]).abs();
        if difference < 1 || difference > 3 {
            return false;
        }

        // Make sure the data continues to increase/decrease
        match increasing {
            None => increasing = Some(report[i] > report[i - 1]),
            Some(increasing) => {
                if (report[i] > report[i - 1]) != increasing {
                    return false;
                }
            }
        }
    }

    true
}

#[aoc(day2, part2)]
pub fn day2_part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for report in input {
        if report_is_safe_with_slow_problem_dampener(&report) {
            sum += 1;
        }
    }

    sum
}

/// Braindead O(n^2) solution
fn report_is_safe_with_slow_problem_dampener(report: &[i32]) -> bool {
    if report_is_safe(&report) {
        return true;
    }

    for i in 0..=report.len() {
        // Remove element at index i from the report
        let a: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, x)| *x)
            .collect();

        println!("{:?} {:?} i = {}", report, a, i);

        if report_is_safe(&a) {
            return true;
        }
    }

    false
}

/// Broken solution
#[warn(dead_code)]
fn _report_is_safe_with_problem_dampener(report: &[i32]) -> bool {
    let mut out_of_range_levels = 0;
    let mut failed_increases = 0;
    let mut failed_decreases = 0;

    for i in 1..report.len() {
        // Make sure the difference between the data points is within the acceptable range
        let difference = (report[i] - report[i - 1]).abs();
        if difference < 1 || difference > 3 {
            out_of_range_levels += 1;
        }

        if report[i] > report[i - 1] {
            // Increasing
            failed_decreases += 1;
        }
        if report[i] < report[i - 1] {
            // Decreasing
            failed_increases += 1
        }
    }

    println!("{report:?} out_of_range_levels: {out_of_range_levels} failed_increases: {failed_increases} failed_decreases: {failed_decreases}");

    return out_of_range_levels + std::cmp::min(failed_increases, failed_decreases) <= 1;
}

#[test]
fn test_day2_part2() {
    let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    assert_eq!(part2(input), 4);
}
