use aoc_runner_derive::aoc_generator;
use sscanf::scanf;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;


type Rule = (String, HashSet<usize>);
type Ticket = Vec<usize>;

#[derive(Debug)]
pub struct Input {
    rules: Vec<Rule>,
    your: Ticket,
    nearby: Vec<Ticket>,
}


#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    let rules_str = sections.next().unwrap();
    let your_str = sections.next().unwrap();
    let nearby_str = sections.next().unwrap();

    let rules: Vec<Rule> = rules_str.lines().map(|line| {
        let a = scanf!(line, "{}: {}-{} or {}-{}", String, usize, usize, usize, usize).unwrap();
        let values: HashSet<usize> =  (a.1 ..= a.2).chain(a.3 ..= a.4).collect();
        (a.0, values)
    }).collect();
    
    let your: Vec<usize> = your_str.lines().skip(1).next().unwrap()
                            .split(',').map(|x| x.parse::<usize>().unwrap())
                            .collect();
    let nearby: Vec<Vec<usize>> = nearby_str.lines().skip(1)
                                    .map(|line| line.split(',')
                                        .map(|x| x.parse::<usize>().unwrap()).collect())
                                    .collect();
    Input{rules, your, nearby}
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let valid = get_valid(&input);

    let total: usize = input.nearby.iter()
                        .map(|ticket| ticket.iter()
                            .filter(|x| !valid.contains(x)))
                            .flatten().sum();
    total
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let valid = get_valid(&input);
    let valid_tickets: Vec<&Ticket> = input.nearby.iter().filter(|ticket| ticket.iter().all(|x| valid.contains(x))).collect();

    for (label, rule) in &input.rules {
        println!("{} {:?}", label, rule.iter().sorted());
        let mut possible: HashSet<usize> = HashSet::from_iter(0..input.your.len());
        for ticket in &input.nearby {
            for (i, value) in ticket.iter().enumerate() {
                if !rule.contains(&value) {
                    println!("    removing {} because of {}", i, value);
                    possible.remove(&i);
                }
            }

        }        
        println!("{:?}", possible);
        
    } 
    0
}

fn get_valid(input: &Input) -> HashSet::<usize> {
    input.rules.iter().map(|(_, set)| set.clone()).flatten().collect() 
}
