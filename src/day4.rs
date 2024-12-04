use aoc_runner_derive::{aoc, aoc_generator};

// pub fn part1(s: &str) -> i32 {
//     let input = parse_input(s);
//     day4_part1(&input)
// }

// pub fn part2(s: &str) -> i32 {
//     let input = parse_input(s);
//     day4_part2(&input)
// }
/*
struct CharacterGrid {
    pub grid: Vec<Vec<char>>
}

impl CharacterGrid {
    pub fn new(rows: usize) -> Self {
        Self {
            grid: Vec::with_capacity(rows)
        }
    }

    pub fn set_row(&mut self, row: usize, s: &str) {
        self.grid[row] = s.chars().into_iter().collect();
    }

    pub fn iter(&self) -> CharacterGridIter {
        CharacterGridIter {
            grid: &self,
            cur_row: 0,
            cur_col: 0,
        }
    }
}

struct CharacterGridIter<'a> {
    grid: &'a CharacterGrid,
    cur_row: usize,
    cur_col: usize,
}

impl<'a> Iterator for CharacterGridIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if true/* if row and column indexes are out of bounds */ {
            return None
        }

        todo!()
    }
}
*/

// #[aoc_generator(day4)]
// fn parse_input(s: &str) -> GeneratorType {
//     let lines: Vec<&str> = s.split("\n").map(|s| s.trim()).collect();

//     let mut

//     for (i, line) in lines.iter().enumerate() {
//         grid.set_row(i, line);
//     }

//     grid
// }

struct Day4Input<'a> {
    pub s: &'a str,

    /// aka the number of columns
    pub row_length: usize,
}
impl<'a> Day4Input<'a> {
    pub fn new(s: &'a str) -> Self {
        let row_length = s.find("\n").expect("Invalid input");

        Self { s, row_length }
    }

    pub fn get_char(&self, row: i32, col: i32) -> char {
        if row < 0 || col < 0 {
            return ' ';
        }
        let row: usize = row.try_into().unwrap();
        let col: usize = col.try_into().unwrap();

        let index: usize = (self.row_length * row) + col + row;

        // if index > self.s.len() {
        //     return ' '
        // }

        match self.s.as_bytes().get(index) {
            Some(byte) => return char::from_u32(*byte as u32).unwrap(),
            None => return ' ',
        }
    }
}

#[aoc(day4, part1)]
pub fn day4_part1(input: &str) -> i32 {
    let mut sum = 0;

    let foo = Day4Input::new(input);

    let directions = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
    ];
    let word = ['X', 'M', 'A', 'S'];

    // Iterate through all the characters in the grid
    let mut row_idx = 0;
    'outer_loop: loop {
        for col_idx in 0..foo.row_length {
            let cur_char = foo.get_char(row_idx, col_idx as i32);

            match cur_char {
                // Search for "XMAS" whenever we find an X
                'X' => {
                    // For each direction
                    for (row_step, col_step) in directions {
                        let mut row = row_idx;
                        let mut col = col_idx as i32;

                        let mut is_match = true;
                        for letter in word {
                            // Check that the current letter is the correct one
                            if foo.get_char(row, col) != letter {
                                is_match = false;
                                break;
                            }
                            row += row_step;
                            col += col_step;
                        }

                        if is_match {
                            sum += 1;
                        }
                    }
                }
                ' ' => break 'outer_loop, // ' ' is returned when the index is out of bounds, so we know we are done iterating when this happens
                _ => (),                  // Ignore all other characters
            }
        }

        row_idx += 1;
    }

    sum
}

#[test]
fn test_day4_part1() {
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    assert_eq!(day4_part1(&input), 18);
}

#[aoc(day4, part2)]
pub fn day4_part2(input: &str) -> i32 {
    todo!()
}
