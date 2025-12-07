use std::{collections::{HashMap, HashSet}, fs, path::absolute};

use anyhow::{Result, anyhow};
use itertools::Itertools;

type Coord = (usize, usize);

fn parse_input(input_lines: &Vec<&str>) -> Result<(Coord, HashSet<Coord>, Coord)> {
    let mut start_point: Option<Coord> = None;
    let mut splitter_coords: HashSet<Coord> = HashSet::new();

    for (y, line) in input_lines.iter().enumerate() {
        for (x, char) in line.char_indices() {
            if char == 'S' {
                if start_point.is_some() {
                    return Err(anyhow!("Found multiple start points?"));
                }
                start_point = Some((x, y));
            } else if char == '^' {
                splitter_coords.insert((x, y));
            }
        }
    }

    if start_point.is_none() {
        return Err(anyhow!("Could not find start point!"));
    }

    let max_coords = (input_lines.len() - 1, input_lines[0].len() - 1);

    Ok((start_point.unwrap(), splitter_coords, max_coords))
}

fn part1(input_lines: &Vec<&str>) -> Result<usize> {
    let mut hit_coords: HashSet<Coord> = HashSet::new();
    let mut beamed_coords: HashSet<Coord> = HashSet::new();
    let mut beam_coords: Vec<Coord> = Vec::new();
    let (start_point, splitter_coords, max_coords) = parse_input(input_lines)?;

    /*
    println!("Starting at {:?}", start_point);
    println!("Splitters at {:?}", splitter_coords);
    println!("Limits at {:?}", max_coords);
    */

    beam_coords.push(start_point);

    while let Some(this_coord) = beam_coords.pop() {
        let next_coord = (this_coord.0, this_coord.1 + 1);

        if beamed_coords.contains(&next_coord) {
            continue;
        }
        else {
            beamed_coords.insert(next_coord);
        }
        
        if splitter_coords.contains(&next_coord) {
            hit_coords.insert(next_coord);
            if next_coord.0 > 0 {
                beam_coords.push((next_coord.0 - 1, next_coord.1));
            }
            if next_coord.0 < max_coords.0 {
                beam_coords.push((next_coord.0 + 1, next_coord.1));
            }
        } else if next_coord.1 < max_coords.1 {
            beam_coords.push(next_coord);
        }
    }

    Ok(hit_coords.len())
}

fn count_splitter_paths(this_splitter: &Coord, splitter_coords: &Vec<Coord>, splitter_paths: &mut HashMap<Coord, usize>) -> usize {
    if let Some(precalculated_paths) = splitter_paths.get(this_splitter) {
        return *precalculated_paths;
    }

    let mut new_coords = vec![(this_splitter.0 + 1, this_splitter.1)];
    if this_splitter.0 > 0 {
        new_coords.push((this_splitter.0 - 1, this_splitter.1));
    }

    let my_paths = new_coords
        .iter()
        .map(|(beam_x, beam_y)|
            match splitter_coords.iter().find(|(x, y)| x == beam_x && y > beam_y) {
                Some(next_splitter) => count_splitter_paths(next_splitter, splitter_coords, splitter_paths),
                None => 1
            }
        )
        .sum::<usize>();

    splitter_paths.insert(*this_splitter, my_paths);

    my_paths
}

fn part2(input_lines: &Vec<&str>) -> Result<usize> {
    let (start_point, mut splitter_coords, _) = parse_input(input_lines)?;
    let mut splitter_paths: HashMap<Coord, usize> = HashMap::new();

    let mut splitter_vec = splitter_coords.drain().collect_vec();
    splitter_vec.sort();

    let result;

    if let Some(first_splitter) = splitter_vec.iter().find(|&&x| x.0 == start_point.0) {
        result = count_splitter_paths(first_splitter, &splitter_vec, &mut splitter_paths)
    }
    else {
        result = 0;
    }

    Ok(result)
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day07.input.txt")?)?;
    let input_lines: Vec<&str> = input_string.lines().collect();

    println!("{}", part1(&input_lines)?);
    println!("{}", part2(&input_lines)?);

    Ok(())
}