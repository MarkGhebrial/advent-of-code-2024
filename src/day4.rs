use aoc_runner_derive::aoc;

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
    let mut sum = 0;

    let foo = Day4Input::new(input);

    // Iterate through all the characters in the grid
    let mut row_idx = 0;
    'outer_loop: loop {
        for col_idx in 0..foo.row_length {
            let cur_char = foo.get_char(row_idx, col_idx as i32);

            match cur_char {
                // Search for an "X-MAS" whenever we find an A
                'A' => {
                    let chars_on_1st_diagonal = (
                        foo.get_char(row_idx + 1, col_idx as i32 + 1),
                        foo.get_char(row_idx - 1, col_idx as i32 - 1),
                    );
                    let chars_on_2nd_diagonal = (
                        foo.get_char(row_idx - 1, col_idx as i32 + 1),
                        foo.get_char(row_idx + 1, col_idx as i32 - 1),
                    );

                    let first_mas = match chars_on_1st_diagonal {
                        ('M', 'S') | ('S', 'M') => true,
                        _ => false,
                    };
                    let second_mas = match chars_on_2nd_diagonal {
                        ('M', 'S') | ('S', 'M') => true,
                        _ => false,
                    };

                    if first_mas && second_mas {
                        sum += 1
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
