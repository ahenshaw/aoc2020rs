use aoc_runner_derive::aoc_generator;
use sscanf::scanf;

type Input = (char, f64);

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        scanf!(line, "{}{}", char, f64).unwrap()
    }).collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Input]) -> f64 {
    let (mut x, mut y) = (0.0, 0.0);
    let mut angle = 0.0;
    for (cmd, units) in input {
        match cmd {
            'R' => angle -= units,
            'L' => angle += units,
            'N' => y += units,
            'E' => x += units,
            'S' => y -= units,
            'W' => x -= units,
            'F' =>  {
                x += angle.to_radians().cos() * units;
                y += angle.to_radians().sin() * units;
            },
            _ => (),
        }
    }
    x.abs() + y.abs()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Input]) -> f64 {
    let (mut x, mut y) = (0.0, 0.0);
    let (mut wx, mut wy) = (10.0f64, 1.0f64);

    for (cmd, units) in input {
        let angle = wy.atan2(wx).to_degrees();
        let dist = (wx * wx + wy * wy).sqrt();
        match cmd {
            'R' => {
                let theta = (angle - units).to_radians();
                wx = dist * theta.cos();
                wy = dist * theta.sin();
            },
            'L' => {
                let theta = (angle + units).to_radians();
                wx = dist * theta.cos();
                wy = dist * theta.sin();
            },
            'N' => wy += units,
            'E' => wx += units,
            'S' => wy -= units,
            'W' => wx -= units,
            'F' =>  {
                x += units * wx; 
                y += units * wy;
            },
            _ => (),
        }
    }
    (x.abs() + y.abs()).round()
}