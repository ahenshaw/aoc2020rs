use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;

type Input = Vec<Vec<char>>;

#[aoc_generator(day6)]
/// Split input into groups and then return each group
/// as a vector of vector of chars
pub fn input_generator(input: &str) -> Vec<Input> {
    input.split("\n\n")
        .map(|group| {
            group.lines().map(|line| {
                line.chars().collect::<Vec<char>>()
            }).collect()
        }).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    input.iter().map(|group|
        group.iter().flatten().collect::<HashSet<&char>>().len()
    ).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    input.iter().map(|group| {
        let mut acc: HashSet<&char> = group[0].iter().collect();
        group[1..].iter()
            .for_each(|line| {
                let lineset: HashSet<&char> = line.iter().collect();
                acc = acc.intersection(&lineset).copied().collect();
            });
        acc.len()
    }).sum()
}
