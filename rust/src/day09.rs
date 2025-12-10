use std::{fs, path::absolute, time::Instant};

use anyhow::{Result, anyhow};
use itertools::Itertools;

type Coord = (u64, u64);

fn parse_input(input_lines: &Vec<&str>) -> Vec<Coord> {
    input_lines
        .iter()
        .map(|&line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .collect()
}

fn part1(input_lines: &Vec<&str>) -> Result<u64> {
    let coords = parse_input(input_lines);

    // Efficiency? Is that something you eat?

    let result = coords
        .iter()
        .combinations(2)
        .map(|vec| {
            let a = vec[0];
            let b = vec[1];

            (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
        })
        .max()
        .unwrap();

    Ok(result)
}

fn compute_area(first_point: Coord, last_point: Coord) -> u64 {
    (first_point.0.abs_diff(last_point.0) + 1) * (first_point.1.abs_diff(last_point.1) + 1)
}

fn part2_brutest_force(input_lines: &Vec<&str>) -> Result<u64> {
    let coords = parse_input(input_lines);

    let midpoints = coords
        .iter()
        .enumerate()
        .map(|(i, point)| {
            let &next_point = coords.get(i + 1).unwrap_or(&coords[0]);

            (
                (point.0 + next_point.0).div_ceil(2),
                (point.1 + next_point.1).div_ceil(2),
            )
        })
        .collect_vec();

    let mut all_combinations = coords
        .iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .collect_vec();

    all_combinations.sort_by(|&a, &b| compute_area(*a.0, *a.1).cmp(&compute_area(*b.0, *b.1)));

    while let Some((&first_point, &second_point)) = all_combinations.pop() {
        let left_side = first_point.0.min(second_point.0);
        let right_side = first_point.0.max(second_point.0);
        let top_side = first_point.1.min(second_point.1);
        let bottom_side = first_point.1.max(second_point.1);

        if !midpoints.iter().chain(coords.iter()).any(|&e| {
            e.0 > left_side && e.0 < right_side && e.1 > top_side && e.1 < bottom_side
        }) {
            return Ok(compute_area(first_point, second_point));
        }
    }

    Err(anyhow!("Found nothing...somehow..."))
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day09.input.txt")?)?;
    let input_lines: Vec<&str> = input_string.lines().collect();

    let now = Instant::now();
    let part1_result = part1(&input_lines)?;
    let part1_time = now.elapsed().as_secs_f64();

    let now = Instant::now();
    let part2_result = part2_brutest_force(&input_lines)?;
    let part2_time = now.elapsed().as_secs_f64();

    println!(
        "Part 1 result: {}, took {} seconds",
        part1_result, part1_time
    );
    println!(
        "Part 2 result: {}, took {} seconds",
        part2_result, part2_time
    );

    Ok(())
}
