use aoc_runner_derive::aoc_generator;
use sscanf::scanf;

#[derive(Debug)]
pub struct Input {
    i: usize,
    j: usize,
    c: char,
    s: Vec<char>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let (i, j, c, s) = scanf!(line, "{}-{} {}: {}", usize, usize, char, String).unwrap();
            Input {
                i,
                j,
                c,
                s: s.chars().collect(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|x| {
            let count = x.s.iter().filter(|&character| *character == x.c).count();
            count >= x.i && count <= x.j
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|x| ((x.s[x.i - 1] == x.c) as usize + (x.s[x.j - 1] == x.c) as usize) == 1)
        .count()
}
