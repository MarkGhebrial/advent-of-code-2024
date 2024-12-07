use std::collections::HashSet;

use super::generator::*;
use aoc_runner_derive::aoc;

pub fn part1(s: &str) -> usize {
    let input = parse_input(s);
    day6_part1(&input)
}

#[aoc(day6, part1)]
pub fn day6_part1(input: &GeneratorType) -> usize {
    let grid = &input.0;
    let mut current_location = input.1;
    let mut current_direction = 0; // Current direction in degrees. 0 is due north

    let mut visited_locations: HashSet<(i32, i32)> = HashSet::new();

    'outer_loop: loop {
        visited_locations.insert(current_location);
        
        loop {
            let next_location = match current_direction % 360 {
                0 => (current_location.0 - 1, current_location.1),
                90 => (current_location.0, current_location.1 + 1),
                180 => (current_location.0 + 1, current_location.1),
                270 => (current_location.0, current_location.1 - 1),
                n => panic!("Unexpected direction: {n}")
            };

            // Break out of the loop if we reach the edge of the grid
            if next_location.0 < 0 || next_location.0 >= grid.len() as i32 || next_location.1 < 0 || next_location.1 >= grid[0].len() as i32 {
                break 'outer_loop;
            }

            // Check the next location
            match grid[next_location.0 as usize][next_location.1 as usize] {
                // Break out of the loop if it's a valid location
                b'^' | b'.' => {
                    current_location = next_location;
                    break;
                },
                // Otherwise turn right
                b'#' => {
                    current_direction += 90;
                }
                b => panic!("Unexpected character: {b}")
            }           
        }
    }

    visited_locations.len()
}

#[test]
fn test_day6_part1() {
    let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    assert_eq!(part1(&input), 41);
}