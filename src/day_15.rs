use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

type Input = usize;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.split(',').map(|x| x.parse::<usize>().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    game(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    game(input, 30_000_000)
}

fn game(input: &[Input], limit: usize) -> usize {
    let mut spoken = HashMap::<usize, Vec<usize>>::new();
    let mut last = 0;

    for i in 0..limit {
        if i < input.len() {
            last = input[i];
        } else {
            let when = spoken.get(&last).unwrap();
            last = if when.len() < 2 {0} else {when[when.len()-1] - when[when.len()-2]};
        }
        spoken.entry(last).or_insert(vec![]).push(i);
    }
    last
}