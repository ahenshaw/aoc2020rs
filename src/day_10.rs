use aoc_runner_derive::aoc_generator;
use sscanf::scanf;
use itertools::{Itertools, sorted};

type Input = usize;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        scanf!(line, "{}", usize).unwrap()
    }).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    let chain = sorted(input);
    let diff: Vec<usize> = chain.tuple_windows::<(_,_)>().map(|(a, b)| b - a).collect();
    (diff.iter().filter(|&x| *x == 1).count() + 1) *
    (diff.iter().filter(|&x| *x == 3).count() + 1)
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Input]) -> u32 {
    for a in input {
    }
    0
}
