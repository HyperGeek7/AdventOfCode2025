use std::{fs, path::absolute};

use anyhow::Result;
use fancy_regex::Regex;

fn part1(line: String) -> Result<u64> {
    let re = Regex::new(r"^([0-9]+)\1$")?;
    let mut invalid_sum = 0;

    for sub_input in line.split(',') {
        let (first, last) = sub_input.split_once('-').unwrap();

        let this_range = (first.parse::<u64>()?)..(last.parse::<u64>()? + 1);
        invalid_sum += this_range
            .filter(|x| re.is_match(&x.to_string()).unwrap())
            .sum::<u64>();
    }

    Ok(invalid_sum)
}

fn part2(line: String) -> Result<u64> {
    let re = Regex::new(r"^([0-9]+)\1+$")?;
    let mut invalid_sum = 0;

    for sub_input in line.split(',') {
        let (first, last) = sub_input.split_once('-').unwrap();

        let this_range = (first.parse::<u64>()?)..(last.parse::<u64>()? + 1);
        invalid_sum += this_range
            .filter(|x| re.is_match(&x.to_string()).unwrap())
            .sum::<u64>();
    }

    Ok(invalid_sum)
}

fn main() -> Result<()> {
    let input_line = fs::read_to_string(absolute("../input_files/day02.input.txt")?)?;

    println!("{}", part1(input_line.clone())?);
    println!("{}", part2(input_line)?);

    Ok(())
}