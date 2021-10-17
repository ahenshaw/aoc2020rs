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
pub fn solve_part2(input: &[Input]) -> usize {
    let mut chain: Vec<&usize> = sorted(input).collect();
    chain.insert(0, &0);
    let diff: Vec<usize> = chain.iter().tuple_windows::<(_,_)>().map(|(&a, &b)| *b - *a).collect();
    diff.split(|x| *x==3)
        .map(|x| x.len())
        .filter(|x| *x > 1)//.collect::<Vec<usize>>());
        .map(|x| (1 << (x-1) as usize) - (x>3) as usize)
        .product()
}
