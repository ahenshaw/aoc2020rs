use aoc_runner_derive::aoc_generator;
use sscanf::scanf;

type Input = i64;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        scanf!(line, "{}", i64).unwrap()
    }).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Input]) -> i64 {
    dbg!(input.len());
    for i in 25..input.len() {
        let target = input[i];
        let previous = &input[i-25..i];
        if !previous.iter().any(|x| previous.contains(&(target - x))) {
            return target;
        }
    }
    0  // shouldn't happen
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Input]) -> i64 {
    const TARGET: i64 = 1639024365;
    let mut start: usize = 0;
    let mut end: usize = 1;
    let mut acc: i64 = input[0] + input[1];
    while acc != TARGET {
        if acc < TARGET {
            end += 1;
            acc += input[end];
        } else {
            acc -= input[start];
            start += 1;
        }
    }
    let r = &input[start..end+1];
    // The answer is the sum of the smallest and largest number in this contiguous range
    r.iter().min().unwrap() + r.iter().max().unwrap()
}
