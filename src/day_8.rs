use aoc_runner_derive::aoc_generator;
use sscanf::scanf;
use std::collections::HashSet;

type Input = (String, i32);

enum Termination {
    Looping(i32),
    Normal(i32)
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        scanf!(line, "{} {}", String, i32).unwrap()
    }).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Input]) -> i32 {
    match runcode(input, -1) {
        Termination::Looping(acc) => acc,
        Termination::Normal(acc) => acc,
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Input]) -> i32 {
    for i in 0..input.len() {
        if let Termination::Normal(acc) = runcode(&input, i as i32) {
            return acc
        }
    }
    0
}

fn runcode(input: &[Input], mutate: i32) -> Termination {
    let mut ip: i32 = 0;
    let mut acc: i32 = 0;
    let mut visited = HashSet::<i32>::new();

    while !visited.contains(&ip) {
        match input.get(ip as usize) {
            Some((opcode, offset)) => {
                visited.insert(ip);
                match opcode.as_str() {
                    "nop" => ip += if ip != mutate {&1} else {offset},
                    "acc" => {ip += 1; acc += offset},
                    "jmp" => ip += if ip != mutate {offset} else {&1},
                    _ => (),
                };
            },
            None => return Termination::Normal(acc),
        }

    }
    Termination::Looping(acc)
}
