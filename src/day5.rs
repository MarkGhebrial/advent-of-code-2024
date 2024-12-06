use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct GeneratorResult {
    pub must_not_occur_before_map: HashMap<i32, HashSet<i32>>,
    pub must_not_occur_after_map: HashMap<i32, HashSet<i32>>,
    pub updates: Vec<Vec<i32>>,
}

pub fn part1(s: &str) -> i32 {
    let input = parse_input(s);
    day5_part1(&input)
}

// pub fn part2(s: &str) -> i32 {
//     let input = parse_input(s);
//     day5_part2(&input)
// }

#[aoc_generator(day5)]
pub fn parse_input(s: &str) -> GeneratorResult {
    // Maps numbers to the numbers that must preceed it
    let mut must_not_occur_before_map: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(1178);
    // Maps numbers to the numbers that must follow it
    let mut must_not_occur_after_map: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(1178);

    let mut updates: Vec<Vec<i32>> = Vec::with_capacity(1200);

    let mut parsing_rules = true; // Indicates whether we are iterating through rules or updates
    for line in s.split("\n") {
        if line == "" {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let first: i32 = line[0..2].parse().unwrap();
            let second: i32 = line[3..].parse().unwrap();

            match must_not_occur_before_map.get_mut(&first) {
                None => {
                    let mut set: HashSet<i32> = HashSet::new();
                    set.insert(second);

                    must_not_occur_before_map.insert(first, set);
                }
                Some(set) => {
                    set.insert(second);
                }
            }
            match must_not_occur_after_map.get_mut(&second) {
                None => {
                    let mut set: HashSet<i32> = HashSet::new();
                    set.insert(first);

                    must_not_occur_after_map.insert(second, set);
                }
                Some(set) => {
                    set.insert(first);
                }
            }
        } else {
            // Parse updates
            let update: Vec<i32> = line
                .trim()
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            updates.push(update);
        }
    }

    GeneratorResult {
        must_not_occur_before_map,
        must_not_occur_after_map,
        updates,
    }
}

#[aoc(day5, part1)]
pub fn day5_part1(input: &GeneratorResult) -> i32 {
    let mut sum = 0;

    // For each update
    // Ensure that all the ordering rules that apply to the update's numbers apply
    for update in input.updates.iter() {
        let mut update_is_in_right_order = true;

        'middle_loop: for (i, num) in update.iter().enumerate() {
            let (before_slice, after_slice) = update.split_at(i);

            // Make sure the number doesn't break any of the before rules
            let must_not_occur_before_set = input.must_not_occur_before_map.get(&num);
            if let Some(set) = must_not_occur_before_set {
                for num2 in before_slice {
                    if set.contains(num2) {
                        update_is_in_right_order = false;
                        break 'middle_loop;
                    }
                }
            }

            // Make sure the number doesn't break any of the after rules
            let must_not_occur_after_set = input.must_not_occur_after_map.get(&num);
            if let Some(set) = must_not_occur_after_set {
                for num2 in after_slice {
                    if set.contains(num2) {
                        update_is_in_right_order = false;
                        break 'middle_loop;
                    }
                }
            }
        }

        if update_is_in_right_order {
            sum += update[update.len() / 2];
        }
    }

    sum
}

#[test]
fn test_day5_part1() {
    let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    assert_eq!(part1(&input), 143);
}

// #[aoc(day5, part2)]
// pub fn day5_part2(input: &GeneratorResult) -> i32 {
//     todo!()
// }
