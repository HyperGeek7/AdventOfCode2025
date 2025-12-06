use std::{collections::HashSet, fs, path::absolute};

use anyhow::Result;

fn parse_input(input_lines: &Vec<&str>) -> Result<(Vec<(u64, u64)>, Vec<u64>)> {
    let break_line_no = input_lines
        .iter()
        .position(|&x| x.is_empty())
        .expect("No blank line found in input!");
    let (range_lines, ingredient_lines) = input_lines.split_at(break_line_no);

    let fresh_ranges = range_lines
        .iter()
        .map(|&x| x.split_once('-').expect("Range line had no dash!"))
        .map(|(low, high)| (low.parse::<u64>().unwrap(), high.parse::<u64>().unwrap()))
        .collect();

    // We end up with the blank line on the front of this vector,
    // so we discard the first element when we're building our iterator.
    let ingredients = ingredient_lines[1..]
        .iter()
        .map(|&x| x.parse::<u64>().unwrap())
        .collect();

    Ok((fresh_ranges, ingredients))
}

fn part1(input_lines: &Vec<&str>) -> Result<usize> {
    let (fresh_ranges, ingredients) = parse_input(input_lines)?;

    let result = ingredients
        .iter()
        .filter(|&&x| {
            fresh_ranges
                .iter()
                .any(|range| range.0 <= x && range.1 >= x)
        })
        .count();

    Ok(result)
}

fn part2(input_lines: &Vec<&str>) -> Result<u64> {
    let (fresh_ranges, _) = parse_input(input_lines)?;

    // There is *definitely* a more rust-y way to do what I'm thinking of here,
    // but right now I'm just trying to get it working at all.

    let mut keep_looping = true;

    let mut these_ranges = HashSet::new();
    for fresh_range in fresh_ranges {
        these_ranges.insert(fresh_range);
    }

    while keep_looping {
        keep_looping = false;

        let mut next_ranges: HashSet<(u64, u64)> = HashSet::new();
        let mut already_handled: HashSet<(u64, u64)> = HashSet::new();

        for range in these_ranges.clone() {
            if already_handled.contains(&range) {
                continue;
            }

            if let Some(overlapping_range) = these_ranges
                .iter()
                .find(|&&x| {
                    x != range
                        && !already_handled.contains(&x)
                        && ((x.0 <= range.0 && x.1 >= range.0)
                            || (x.0 <= range.1 && x.1 >= range.1))
                })
            {
                /*
                println!(
                    "Overlap between ranges {:?} and {:?}",
                    range, overlapping_range
                );
                 */
                let new_range = (
                    range.0.min(overlapping_range.0),
                    range.1.max(overlapping_range.1),
                );
                //println!("New range is {:?}", new_range);
                next_ranges.insert(new_range);
                already_handled.insert(range);
                already_handled.insert(*overlapping_range);
                keep_looping = true;
            }
            else {
                already_handled.insert(range);
                next_ranges.insert(range);
            }
        }

        these_ranges = next_ranges;
    }

    let result: u64 = these_ranges
        .iter()
        .map(|(low, high)| *high - *low + 1)
        .sum();

    Ok(result)
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day05.input.txt")?)?;
    let input_lines: Vec<&str> = input_string.lines().collect();

    println!("{}", part1(&input_lines)?);
    println!("{}", part2(&input_lines)?);

    Ok(())
}
