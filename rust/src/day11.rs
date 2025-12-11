use std::{cell::RefCell, collections::HashMap, fs, path::absolute, time::Instant};

use anyhow::Result;

struct WireNode {
    name: String,
    paths_out: Option<usize>,
    exits: Vec<String>,
    milestone_paths_out: HashMap<[bool; 2], usize>,
}

impl WireNode {
    fn get_paths_out(&mut self, node_map: &HashMap<String, RefCell<WireNode>>) -> usize {
        if self.paths_out.is_some() {
            self.paths_out.unwrap()
        } else if self.exits.contains(&String::from("out")) {
            self.paths_out = Some(1);
            1
        } else {
            let total_paths_out = self
                .exits
                .iter()
                .map(|e| {
                    node_map
                        .get(e)
                        .unwrap()
                        .borrow_mut()
                        .get_paths_out(node_map)
                })
                .sum::<usize>();
            self.paths_out = Some(total_paths_out);
            total_paths_out
        }
    }

    fn get_paths_out_with_milestones(
        &mut self,
        node_map: &HashMap<String, RefCell<WireNode>>,
        hit_dac: &bool,
        hit_fft: &bool,
    ) -> usize {
        if self.exits.contains(&String::from("out")) {
            if *hit_dac && *hit_fft {
                return 1;
            } else {
                return 0;
            }
        }

        let my_hit_dac = if self.name == "dac" { true } else { *hit_dac };

        let my_hit_fft = if self.name == "fft" { true } else { *hit_fft };

        let milestone_key = [my_hit_dac, my_hit_fft];

        if let Some(milestone_paths) = self.milestone_paths_out.get(&milestone_key) {
            return *milestone_paths;
        }

        let total_paths_out = self
            .exits
            .iter()
            .map(|e| {
                node_map
                    .get(e)
                    .unwrap()
                    .borrow_mut()
                    .get_paths_out_with_milestones(node_map, &my_hit_dac, &my_hit_fft)
            })
            .sum::<usize>();

        self.milestone_paths_out
            .insert(milestone_key, total_paths_out);
        total_paths_out
    }
}

fn parse_input(input_lines: &Vec<&str>) -> HashMap<String, RefCell<WireNode>> {
    let mut node_map = HashMap::new();

    for line in input_lines {
        let mut line_components = line.split_whitespace();
        let node_name = line_components.next().unwrap().strip_suffix(":").unwrap();
        let mut exits = Vec::new();
        for exit in line_components.map(|e| e.to_string()) {
            exits.push(exit);
        }
        let this_node = WireNode {
            name: node_name.to_string(),
            paths_out: None,
            milestone_paths_out: HashMap::new(),
            exits,
        };

        node_map.insert(node_name.to_string(), RefCell::new(this_node));
    }

    node_map
}

fn part1(input_lines: &Vec<&str>) -> Result<usize> {
    let node_map = parse_input(input_lines);
    let result = node_map
        .get("you")
        .unwrap()
        .borrow_mut()
        .get_paths_out(&node_map);

    Ok(result)
}

fn part2(input_lines: &Vec<&str>) -> Result<usize> {
    let node_map = parse_input(input_lines);
    let result = node_map
        .get("svr")
        .unwrap()
        .borrow_mut()
        .get_paths_out_with_milestones(&node_map, &false, &false);

    Ok(result)
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day11.input.txt")?)?;
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
