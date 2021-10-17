use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use regex::Regex;

type Input = std::collections::HashMap<String, String>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.split("\n\n").map(|record| {
        record.split_ascii_whitespace().map(|item| {
            item.splitn(2, ':')
            .map(|s| s.to_owned())
            .next_tuple().unwrap()
        }).filter(|(key, _)| key != "cid"). collect()
    }).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    input.iter().filter(|record| {
        record.keys().count() == 7
    }).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    input.iter().filter(|r| {
       byr(r) && iyr(r) && eyr(r) && hcl(r) && pid(r) && ecl(r) && hgt(r)
    }).count()
}

fn byr(input: &Input) -> bool {
    match input.get("byr").cloned() {
        Some(field) => {
            match field.parse::<u32>() {
                Ok(x) => (1920..=2002).contains(&x),
                _ => false,
            }
        },
        _ => false,
    }
}

fn iyr(input: &Input) -> bool {
    match input.get("iyr").cloned() {
        Some(field) => {
            match field.parse::<u32>() {
                Ok(x) => (2010..=2020).contains(&x),
                _ => false,
            }
        },
        _ => false,
    }
}

fn eyr(input: &Input) -> bool {
    match input.get("eyr").cloned() {
        Some(field) => {
            match field.parse::<u32>() {
                Ok(x) => (2020..=2030).contains(&x),
                _ => false,
            }
        },
        _ => false,
    }
}

fn hcl(input: &Input) -> bool {
    let re = Regex::new(r"^#[\dabcdef]{6}$").unwrap();
    match input.get("hcl").cloned() {
        Some(field) => re.is_match(&field),
        _ => false,
    }   
}

fn pid(input: &Input) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    match input.get("pid").cloned() {
        Some(field) => re.is_match(&field),
        _ => false,
    }   
}

fn ecl(input: &Input) -> bool {
    let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    match input.get("ecl").cloned() {
        Some(field) => re.is_match(&field),
        _ => false,
    }   
}

fn hgt(input: &Input) -> bool {
    let re = Regex::new(r"^(?P<x>\d+)(?P<unit>in|cm)$").unwrap();

    match input.get("hgt").cloned() {
        Some(field) => {
            match re.captures(&field) {
                Some(cap) => {
                    let x = cap.name("x").unwrap().as_str().parse::<u32>().unwrap();
                    match cap.name("unit").unwrap().as_str() {
                        "in" => (59..=76).contains(&x),
                        "cm" => (150..=193).contains(&x),
                        _ => false,
                    }
                },
                None => false,
            }
        },
        _ => false,
    }   
}
