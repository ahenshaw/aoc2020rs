use aoc_runner_derive::aoc_generator;

type Input = (usize, Vec<Option<usize>>);

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<usize>().unwrap();
    let buses: Vec<Option<usize>> = lines.next().unwrap()
        .split(',')
        .map(|x| if x == "x" {None} else {Some(x.parse::<usize>().unwrap())}).collect();
    (timestamp, buses)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let (ts, buses) = input;
    // Remove the missing buses
    let buses: Vec<&usize> = buses.iter().filter_map(|b| b.as_ref()).collect();
    let wait: Vec<usize> = buses.iter().map(|x| *x - (*ts % *x)).collect();
    let min_wait = wait.iter().min().unwrap();
    let index = wait.iter().position(|x| x == min_wait).unwrap();
    buses[index] * *min_wait
}

#[aoc(day13, part2)]

pub fn solve_part2(input: &Input) -> i64 {
    let (_, buses) = input;
    let mut residues = Vec::<i64>::new();
    let mut modulii  = Vec::<i64>::new();

    for (i, bus) in buses.iter().enumerate() {
        if let Some(bus) = bus {
            let bus = *bus as i64;
            let i = i as i64;
            residues.push((bus - i) % bus);
            modulii.push(bus);
        }
    }
    chinese_remainder(&residues, &modulii).unwrap()
}


// Code for Chinese Remainder Theorem from 
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}
 