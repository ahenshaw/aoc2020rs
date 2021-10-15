use aoc_runner_derive::aoc_generator;

type Input = Vec<char>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        line.chars().collect()
    }).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Input]) -> u32 {
    num_trees(input, &3, &1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Input]) -> u32 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |prod, (dx, dy)| {
            prod * num_trees(input, dx, dy)
        })
}

fn num_trees(input: &[Input], dx: &usize, dy: &usize) -> u32 {
    let mut col: usize = 0;
    let mut row: usize = 0;
    let mut count = 0u32;
    while row < input.len() {
        col = col % input[row].len();
        count += (input[row][col] == '#') as u32;
        col += dx;
        row += dy;
    }
    count
}

