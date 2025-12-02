use std::{fs, path::absolute};

use anyhow::Result;
use fancy_regex::Regex;

fn part1(line: String) -> Result<u64> {
    let re = Regex::new(r"^([0-9]+)\1$")?;

    let invalid_sum = line
        .split(',')
        .map(|x| x.split_once('-').unwrap())
        .map(|(first, last)| (first.parse::<u64>().unwrap())..(last.parse::<u64>().unwrap() + 1))
        .flatten()
        .filter(|x| re.is_match(&x.to_string()).unwrap())
        .sum::<u64>();

    Ok(invalid_sum)
}

fn part2(line: String) -> Result<u64> {
    let re = Regex::new(r"^([0-9]+)\1+$")?;

    let invalid_sum = line
        .split(',')
        .map(|x| x.split_once('-').unwrap())
        .map(|(first, last)| (first.parse::<u64>().unwrap())..(last.parse::<u64>().unwrap() + 1))
        .flatten()
        .filter(|x| re.is_match(&x.to_string()).unwrap())
        .sum::<u64>();

    Ok(invalid_sum)
}

fn main() -> Result<()> {
    let input_line = fs::read_to_string(absolute("../input_files/day02.input.txt")?)?;

    println!("{}", part1(input_line.clone())?);
    println!("{}", part2(input_line)?);

    Ok(())
}
