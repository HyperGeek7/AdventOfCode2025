use std::{cell::RefCell, fs, path::absolute, time::Instant};

use anyhow::Result;
use itertools::Itertools;

const ITER_COUNT: usize = 10;

#[derive(Clone, Debug)]
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

struct JunctionBox {
    pos: Coord3D,
    circuit: Option<usize>,
}

fn part1(input_lines: &Vec<&str>) -> Result<u64> {
    let mut boxes = Vec::new();
    let mut box_pairs = Vec::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    for line in input_lines {
        let mut line_components = line.split(",");
        let x = line_components.next().unwrap().parse::<i64>()?;
        let y = line_components.next().unwrap().parse::<i64>()?;
        let z = line_components.next().unwrap().parse::<i64>()?;
        let coord = Coord3D { x, y, z };
        boxes.push(RefCell::new(JunctionBox{pos: coord, circuit: None}));
    }

    for (i, box_a) in boxes.iter().enumerate() {
        for (j, box_b) in boxes[(i+1)..].iter().enumerate() {
            let box_a = box_a.borrow();
            let box_b = box_b.borrow();
            box_pairs.push((i, j+i+1, box_a.pos.distance_to(&box_b.pos)));
        }
    }

    box_pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    box_pairs.truncate(ITER_COUNT);

    while let Some(next_pair) = box_pairs.pop() {
        let mut box_a = boxes[next_pair.0].borrow_mut();
        let mut box_b = boxes[next_pair.1].borrow_mut();

        if let Some(circuit_a) = box_a.circuit {
            if let Some(circuit_b) = box_b.circuit {
                if circuit_a == circuit_b {
                    continue;
                }
                else {
                    let mut drain_circuit = circuits.get_mut(circuit_b).unwrap().drain(..).collect_vec();
                    circuits.get_mut(circuit_a).unwrap().append(&mut drain_circuit);
                }
            }
            else {
                box_b.circuit = Some(circuit_a);
                circuits.get_mut(circuit_a).unwrap().push(next_pair.1);
            }
        }
        else if let Some(circuit_b) = box_b.circuit {
            box_a.circuit = Some(circuit_b);
            circuits.get_mut(circuit_b).unwrap().push(next_pair.0);
        }
        else {
            box_a.circuit = Some(circuits.len());
            box_b.circuit = Some(circuits.len());
            let mut new_circuit: Vec<usize> = Vec::new();
            new_circuit.push(next_pair.0);
            new_circuit.push(next_pair.1);
            circuits.push(new_circuit);
        }
    }

    let mut circuit_sizes = circuits.iter().map(|e| e.len() as u64).collect_vec();
    circuit_sizes.sort();

    Ok(circuit_sizes.pop().unwrap() * circuit_sizes.pop().unwrap() * circuit_sizes.pop().unwrap())
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day08.demo.txt")?)?;
    let input_lines: Vec<&str> = input_string.lines().collect();

    let now = Instant::now();
    let part1_result = part1(&input_lines)?;
    let part1_time = now.elapsed().as_secs_f64();

    println!(
        "Part 1 result: {}, took {} seconds",
        part1_result, part1_time
    );

    /*
    let now = Instant::now();
    let part2_result = part2_brutest_force(&input_lines)?;
    let part2_time = now.elapsed().as_secs_f64();

    println!(
        "Part 2 result: {}, took {} seconds",
        part2_result, part2_time
    );
     */

    Ok(())
}
