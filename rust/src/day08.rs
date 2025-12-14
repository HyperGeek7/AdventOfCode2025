use std::{cell::RefCell, fs, path::absolute, time::Instant};

use anyhow::{Result, anyhow};
use itertools::Itertools;

const ITER_COUNT: usize = 1000;

#[derive(Debug, Clone, Copy)]
struct Coord3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord3D {
    fn distance_to(&self, other: &Coord3D) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    pos: Coord3D,
    circuit: usize,
}

type InputSet = (Vec<JunctionBox>, Vec<(usize, usize, f64)>, Vec<Vec<usize>>);

fn parse_input(input_lines: &Vec<&str>) -> Result<InputSet> {
    let mut boxes = Vec::new();
    let mut box_pairs = Vec::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    for line in input_lines {
        let mut line_components = line.split(",");
        let x = line_components.next().unwrap().parse::<i64>()?;
        let y = line_components.next().unwrap().parse::<i64>()?;
        let z = line_components.next().unwrap().parse::<i64>()?;
        let coord = Coord3D { x, y, z };
        let circuit_num = circuits.len();
        let mut new_circuit = Vec::new();
        boxes.push(JunctionBox {
            pos: coord,
            circuit: circuit_num,
        });
        new_circuit.push(circuit_num);
        circuits.push(new_circuit);
    }

    for mut box_combo in boxes.iter().enumerate().combinations(2) {
        let (i, box_a) = box_combo.pop().unwrap();
        let (j, box_b) = box_combo.pop().unwrap();
        box_pairs.push((i, j, box_a.pos.distance_to(&box_b.pos)));
    }

    box_pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    Ok((boxes, box_pairs, circuits))
}

fn part1((in_boxes, mut box_pairs, mut circuits): InputSet) -> Result<u64> {
    let boxes = in_boxes.iter().map(|e| RefCell::new(*e)).collect_vec();
    box_pairs.truncate(ITER_COUNT);
    box_pairs.reverse();

    while let Some(next_pair) = box_pairs.pop() {
        let box_a = boxes[next_pair.0].borrow_mut();
        let mut box_b = boxes[next_pair.1].borrow_mut();

        let circuit_a = box_a.circuit;
        let circuit_b = box_b.circuit;

        if circuit_a == circuit_b {
            continue;
        } else {
            let mut drain_circuit = circuits.get_mut(circuit_b).unwrap().drain(..).collect_vec();
            // box_b is already borrowed as mutable, so we need to not try and set it in the loop.
            box_b.circuit = box_a.circuit;
            for circuit in drain_circuit.iter() {
                if *circuit == next_pair.1 {
                    continue;
                }
                let mut this_box = boxes[*circuit].borrow_mut();
                this_box.circuit = box_a.circuit;
            }
            circuits
                .get_mut(circuit_a)
                .unwrap()
                .append(&mut drain_circuit);
        }
    }

    let mut circuit_sizes = circuits
        .iter()
        .map(|e| e.len() as u64)
        .filter(|&e| e > 0)
        .collect_vec();
    circuit_sizes.sort();

    Ok(circuit_sizes.pop().unwrap()
        * circuit_sizes.pop().unwrap_or(1)
        * circuit_sizes.pop().unwrap_or(1))
}

fn part2((in_boxes, mut box_pairs, mut circuits): InputSet) -> Result<i64> {
    let boxes = in_boxes.iter().map(|e| RefCell::new(*e)).collect_vec();
    let mut circuit_count = circuits.len();

    box_pairs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    while let Some(next_pair) = box_pairs.pop() {
        let box_a = boxes[next_pair.0].borrow_mut();
        let mut box_b = boxes[next_pair.1].borrow_mut();

        let circuit_a = box_a.circuit;
        let circuit_b = box_b.circuit;

        if circuit_a == circuit_b {
            continue;
        } else {
            if circuit_count == 2 {
                // We are about to join the last two circuits.
                // This is our terminating condition.

                return Ok(box_a.pos.x * box_b.pos.x);
            }

            let mut drain_circuit = circuits.get_mut(circuit_b).unwrap().drain(..).collect_vec();
            // box_b is already borrowed as mutable, so we need to not try and set it in the loop.
            box_b.circuit = box_a.circuit;
            for circuit in drain_circuit.iter() {
                if *circuit == next_pair.1 {
                    continue;
                }
                let mut this_box = boxes[*circuit].borrow_mut();
                this_box.circuit = box_a.circuit;
            }
            circuits
                .get_mut(circuit_a)
                .unwrap()
                .append(&mut drain_circuit);
            circuit_count -= 1;
        }
    }

    Err(anyhow!(
        "You somehow joined every pair of circuits without forming 1 circuit.\nExplain yourself.\nNow."
    ))
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day08.input.txt")?)?;
    let input_lines: Vec<&str> = input_string.lines().collect();

    let now: Instant = Instant::now();
    let parsed_input = parse_input(&input_lines)?;
    let parsing_time = now.elapsed().as_secs_f64();

    println!("Data parsed in {} seconds", parsing_time);

    let now = Instant::now();
    let part1_result = part1(parsed_input.clone())?;
    let part1_time = now.elapsed().as_secs_f64();

    println!(
        "Part 1 result: {}, took {} seconds",
        part1_result, part1_time
    );

    let now = Instant::now();
    let part2_result = part2(parsed_input.clone())?;
    let part2_time = now.elapsed().as_secs_f64();

    println!(
        "Part 2 result: {}, took {} seconds",
        part2_result, part2_time
    );

    Ok(())
}
