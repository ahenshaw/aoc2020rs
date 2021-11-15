use aoc_runner_derive::aoc_generator;
use sscanf::scanf;

pub enum Op {
    Mask(String),
    Mem((usize, usize)),
}

type Input = Op;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        let (left, right) = scanf!(line, "{} = {}", String, String).unwrap();
        if left.starts_with("mask") {
            Op::Mask(right)
        } else {
            let addr = scanf!(left, "mem[{}]", usize).unwrap();
            let value = right.parse::<usize>().unwrap();
            Op::Mem((addr, value))
        }
    }).collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    let mut mem = std::collections::HashMap::<usize, usize>::new();
    let mut ones: usize = 0;
    let mut zeros: usize = 0;

    for op in input {
        match op {
            Op::Mask(mask) => {
                ones  = usize::from_str_radix(&mask.replace('X', "1"), 2).unwrap();
                zeros = usize::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
            },
            Op::Mem((addr, value)) =>  {
                mem.insert(*addr, (value | zeros) & ones);
            },
        }
    }
    mem.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    let mut mem = std::collections::HashMap::<usize, usize>::new();
    let mut mask = String::new();

    for op in input {
        match op {
            Op::Mask(x) => {mask = x.to_string();},
            Op::Mem((addr, value)) =>  {
                let newmask: String = mask.chars().enumerate().map(|(i, x)| {
                    let bit = if (addr >> (mask.len() - i - 1)) & 1 == 1 {'1'} else {'0'} ;
                    if x == '0' {bit} else {x}
                }).collect();
                let mut addresses = Vec::<String>::new();
                wildcard(&mut addresses, newmask);
                for address in addresses {
                    let addr = usize::from_str_radix(&address, 2).unwrap();
                    mem.insert(addr, *value);
                }
            },
        }
    }
    mem.values().sum()
}

/// Recursively xpand each wildcard 'X' as two new addresses (0 and 1)
fn wildcard(addresses: &mut Vec<String>, addr: String) {
    if addr.contains(&"X") {
        wildcard(addresses, addr.replacen('X', "0", 1));
        wildcard(addresses, addr.replacen('X', "1", 1));
    } else {
        addresses.push(addr);
    }
}
