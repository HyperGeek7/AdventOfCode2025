use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::BitXorAssign,
    path::absolute,
    time::Instant,
};

use anyhow::Result;
use itertools::Itertools;
use microlp::{ComparisonOp, LinearExpr, Problem};

#[derive(Debug)]
struct LightPuzzle {
    target_state: usize,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl LightPuzzle {
    fn solve_lights(&self) -> usize {
        let mut node_states = HashMap::new();
        let mut unvisited_set: HashSet<usize> = HashSet::new();
        let base_state = 0;
        node_states.insert(base_state, 0_usize);
        unvisited_set.insert(base_state);

        loop {
            let (shortest_node, path_length) = node_states
                .iter()
                .filter(|(k, _)| unvisited_set.contains(k))
                .min_by_key(|(_, v)| **v)
                .unwrap();
            let next_path_length = path_length.to_owned() + 1;
            let next_node = shortest_node.to_owned();
            unvisited_set.remove(&next_node);

            for button_set in self.buttons.iter() {
                let mut this_state = next_node.to_owned();
                for &button in button_set {
                    this_state.bitxor_assign(2_usize.pow(button as u32));
                }

                if this_state == self.target_state {
                    return next_path_length;
                }

                if let Some(existing_length) = node_states.get(&this_state) {
                    if *existing_length > next_path_length {
                        node_states.insert(this_state, next_path_length);
                    }
                } else {
                    node_states.insert(this_state, next_path_length);
                    unvisited_set.insert(this_state);
                }
            }
        }
    }

    fn solve_joltage(&self) -> usize {
        let mut jolt_constraints: HashMap<usize, LinearExpr> = HashMap::new();
        let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);

        for button_set in self.buttons.iter() {
            let this_var = problem.add_integer_var(1., (0, i32::MAX));

            for jolt_out in button_set {
                jolt_constraints
                    .entry(*jolt_out)
                    .or_insert(LinearExpr::empty())
                    .add(this_var, 1.);
            }
        }

        for (i, target_joltage) in self.joltage.iter().enumerate() {
            if jolt_constraints.contains_key(&i) {
                problem.add_constraint(
                    jolt_constraints.get(&i).unwrap().clone(),
                    ComparisonOp::Eq,
                    *target_joltage as f64,
                );
            }
        }

        let solution = problem.solve().unwrap();
        solution
            .iter()
            .map(|(_, value)| {
                // Ah, floating point imprecision,
                // my old nemesis.
                value.round() as usize
            })
            .sum::<usize>()
    }
}

fn parse_input(input_lines: &Vec<&str>) -> Vec<LightPuzzle> {
    let mut result = Vec::new();

    for line in input_lines {
        let mut target_state: usize = 0;
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let mut joltage: Vec<usize> = Vec::new();

        let line_components = line.split_whitespace();

        for component in line_components {
            if component.starts_with("[") {
                target_state = component
                    .strip_prefix("[")
                    .unwrap_or(component)
                    .strip_suffix("]")
                    .unwrap_or(component)
                    .char_indices()
                    .filter_map(|(i, e)| match e {
                        '#' => Some(2_usize.pow(i as u32)),
                        _ => None,
                    })
                    .sum::<usize>();
            } else if component.starts_with("(") {
                buttons.push(
                    component
                        .strip_prefix("(")
                        .unwrap_or(component)
                        .strip_suffix(")")
                        .unwrap_or(component)
                        .split(",")
                        .map(|e| e.parse::<usize>().unwrap())
                        .collect_vec(),
                );
            } else if component.starts_with("{") {
                joltage = component
                    .strip_prefix("{")
                    .unwrap_or(component)
                    .strip_suffix("}")
                    .unwrap_or(component)
                    .split(",")
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect_vec();
            }
        }
        result.push(LightPuzzle {
            target_state,
            buttons,
            joltage,
        });
    }

    result
}

fn part1(input_lines: &Vec<&str>) -> Result<usize> {
    let puzzles = parse_input(input_lines);

    let result = puzzles
        .iter()
        .map(|puzzle| puzzle.solve_lights())
        .sum::<usize>();

    Ok(result)
}

fn part2(input_lines: &Vec<&str>) -> Result<usize> {
    let puzzles = parse_input(input_lines);

    let result = puzzles
        .iter()
        .map(|puzzle| puzzle.solve_joltage())
        .sum::<usize>();

    Ok(result)
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day10.input.txt")?)?;
    let input_lines: Vec<&str> = input_string.lines().collect();

    let now = Instant::now();
    let part1_result = part1(&input_lines)?;
    let part1_time = now.elapsed().as_secs_f64();

    println!(
        "Part 1 result: {}, took {} seconds",
        part1_result, part1_time
    );

    let now = Instant::now();
    let part2_result = part2(&input_lines)?;
    let part2_time = now.elapsed().as_secs_f64();

    println!(
        "Part 2 result: {}, took {} seconds",
        part2_result, part2_time
    );

    Ok(())
}
