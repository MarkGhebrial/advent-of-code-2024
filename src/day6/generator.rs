use aoc_runner_derive::aoc_generator;

pub type GeneratorType = (Vec<Vec<u8>>, (i32, i32));

#[aoc_generator(day6)]
pub fn parse_input(s: &str) -> GeneratorType {

    let mut out: Vec<Vec<u8>> = Vec::with_capacity(130);
    let mut location_of_guard: (i32, i32) = (0, 0);

    for (i, line) in s.split("\n").enumerate() {
        let mut bytes: Vec<u8> = Vec::with_capacity(130);
        for (j, byte) in line.bytes().enumerate() {
            if byte == b'^' {
                location_of_guard = (i as i32, j as i32);
            }
            bytes.push(byte);
        }
        out.push(bytes);
    }

    (out, location_of_guard)
}