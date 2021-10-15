use aoc_runner_derive::aoc_generator;
use sscanf::scanf;

#[derive(Debug)]
pub struct Input {
    a: usize,
}

#[aoc_generator(day#)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        let (a) = scanf!(line, "{}", usize).unwrap();
        Input{a}
    }).collect()
}

#[aoc(day#, part1)]
pub fn solve_part1(input: &[Input]) -> u32 {
    for a in input {
    }
    0
}

#[aoc(day#, part2)]
pub fn solve_part2(input: &[Input]) -> u32 {
    for a in input {
    }
    0
}
