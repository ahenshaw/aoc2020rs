use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;

type Input = u32;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        u32::from_str_radix(&line
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1"), 2).unwrap()
    }).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Input]) -> u32 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Input]) -> u32 {
    let visible: HashSet<u32> = input.iter().copied().collect();
    let max = *input.iter().max().unwrap();
    let all: HashSet<u32> = (1..max).collect();
    for &missing in all.difference(&visible) {
        if visible.contains(&(missing - 1)) && visible.contains(&(missing + 1)) {
            return missing
        }
    }
    0
}
