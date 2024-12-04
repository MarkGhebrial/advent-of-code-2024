use aoc_runner_derive::aoc;
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

#[aoc(day3, part2)]
pub fn part2(s: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"(mul|do|don't)(\((\d+),(\d+)\)|\(\))").unwrap();

    let mut dont = false;
    for capture in re.captures_iter(&s) {
        match &capture[1] {
            "mul" => {
                if dont {
                    continue;
                }
                if let (Some(x), Some(y)) = (&capture.get(3), &capture.get(4)) {
                    sum += x.as_str().parse::<i32>().unwrap() * y.as_str().parse::<i32>().unwrap();
                }
            }
            "do" => {
                if let (None, None) = (&capture.get(3), &capture.get(4)) {
                    dont = false;
                }
            }
            "don't" => {
                if let (None, None) = (&capture.get(3), &capture.get(4)) {
                    dont = true;
                }
            }
            _ => panic!(),
        }
    }

    sum
}

#[test]
fn test_day3_part2() {
    let input =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64]\n(mul(11,8)undo()?mul(8,5))mul()\n";

    assert_eq!(part2(&input), 48);
}
