use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use counter::Counter;

#[derive(Debug, Clone)]
pub struct Input {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Input {
    fn get(&self, x: isize, y: isize) -> char {
        if (0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y) {
            self.grid[y as usize][x as usize]
        } else {
            '_'
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<char> {
        (-1 .. 2)
            .cartesian_product(-1 .. 2)
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            .map(|(dx, dy)| self.get(x as isize + dx, y as isize + dy))
            .collect()
    }

    fn visible(&self, x: usize, y: usize) -> Vec<char> {
        [0, 1, 2, 3, 5, 6, 7, 8].iter().map(|d| {
            let (dx, dy) = (d/3, d % 3);
            let mut x = x as isize;
            let mut y= y as isize;
            loop {
                x += dx - 1;
                y += dy - 1;
                let pos = self.get(x, y);
                if pos != '.' {
                    break pos
                }
            }
        }).collect()
    }

    fn rule1(&self, x: usize, y: usize) -> (char, char) {
        let cell = self.get(x as isize, y as isize);
        let counts = self.neighbors(x, y).into_iter().collect::<Counter<char>>();
        let occupied = counts[&'#'];

        let new_cell = match cell {
            'L' => {if occupied == 0 {'#'} else {'L'}},
            '#' => {if occupied >= 4 {'L'} else {'#'}},
            _ => '.',
        };
        (cell, new_cell)
    }

    fn rule2(&self, x: usize, y: usize) -> (char, char) {
        let cell = self.get(x as isize, y as isize);
        let counts = self.visible(x, y).into_iter().collect::<Counter<char>>();
        let occupied = counts[&'#'];

        let new_cell = match cell {
            'L' => {if occupied == 0 {'#'} else {'L'}},
            '#' => {if occupied >= 5 {'L'} else {'#'}},
            _ => '.',
        };
        (cell, new_cell)
    }

    fn update(&mut self, rule: impl Fn(&Self, usize, usize) -> (char, char)) -> bool 
    where 
    {
        let mut grid = self.grid.clone();
        let mut dirty = false;
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                let (old, new) = rule(self, x, y);
                grid[y][x] = new;
                if old != new {
                    dirty = true;
                }
            }
        }
        self.grid = grid;
        dirty
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    let grid: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    let width = grid[0].len();
    let height = grid.len();
    Input{grid, width, height}
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut ca = input.clone();
    loop {
        if !ca.update(Input::rule1) {
            break;
        }
    }
    ca.grid.into_iter().flat_map(|row| row.into_iter()).filter(|x| *x == '#').count()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut ca = input.clone();
    loop {
        if !ca.update(Input::rule2) {
            break;
        }
    }
    ca.grid.into_iter().flat_map(|row| row.into_iter()).filter(|x| *x == '#').count()
}
