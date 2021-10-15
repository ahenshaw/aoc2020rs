use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    for a in input {
        for b in input {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    for a in input {
        for b in input {
            for c in input {
                if a + b + c == 2020 {
                    return a * b * c;
                }
                if a + b >= 2020 {
                    break;
                }
            }
        }
    }
    0
}
