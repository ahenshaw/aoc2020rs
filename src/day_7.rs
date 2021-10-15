use aoc_runner_derive::aoc_generator;
use sscanf::scanf;
use std::collections::{HashSet, HashMap};

type Input = (String, Vec<(usize, String)>);

// dim silver bags contain 2 shiny chartreuse bags, 4 dull magenta bags.
#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        let (outer, inner) = scanf!(line, "{} bags contain {}.", String, String).unwrap();
        if inner == "no other bags" {
            (outer, vec![])
        } else {
            let inside: Vec<(usize, String)> = inner
                .split(", ")
                .map(|s| scanf!(s.replace("bags", "bag"), "{} {} bag", usize, String).unwrap())
                .collect();
            (outer, inside)
        }
    }).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    let mut parents = HashMap::<String, HashSet<String>>::new();
    for (parent, bags) in input {
        for (_, bag) in bags {
            if !parents.contains_key(bag) {
                parents.insert(bag.clone(), HashSet::new());
            }
            parents.get_mut(bag).unwrap().insert(parent.clone());
        }
    }
    let mut bags = HashSet::<String>::new();
    bags = all_parents(&parents, &"shiny gold".to_owned(), bags);
    bags.len()
}

fn all_parents(parents: &HashMap::<String, HashSet<String>>, bag: &String, mut bags: HashSet<String>)  -> HashSet<String> {
    if let Some(parent_set) = parents.get(bag) {
        for parent in parent_set {
            bags.insert(parent.clone());
            bags = all_parents(&parents, parent, bags);
        }
    }
    bags
}

fn count_bags(parents: &HashMap::<String, Vec<(usize, String)>>, bag: String) -> usize {
    let inside_count = match parents.get(&bag) {
        Some(v) => v.iter().map(|(count, child)| count * count_bags(parents, child.clone())).sum(),
        None => 0,
    };
    inside_count + 1
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    let parents: HashMap::<String, Vec<(usize, String)>> = input.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    count_bags(&parents, "shiny gold".to_owned()) - 1
}
