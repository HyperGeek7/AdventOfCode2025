use anyhow::{Result, anyhow};
use std::{fs, path::absolute};

fn part1(lines: Vec<&str>) -> Result<i32> {
    let mut pos = 50;
    let mut zero_count = 0;

    for line in lines {
        let (dir, amt) = line.split_at(1);
        let amt = amt.parse::<i32>()?;

        let dir = match dir {
            "L" => Ok(-1),
            "R" => Ok(1),
            _ => Err(anyhow!("Dials only go left and right.")),
        }?;

        pos += amt * dir;
        pos %= 100;

        if pos == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count)
}

fn part2(lines: Vec<&str>) -> Result<i32> {
    let mut pos = 50;
    let mut zero_count = 0;

    for line in lines {
        let starting_pos = pos;
        let (dir, amt) = line.split_at(1);
        let amt = amt.parse::<i32>()?;

        let dir = match dir {
            "L" => Ok(-1),
            "R" => Ok(1),
            _ => Err(anyhow!("Dials only go left and right.")),
        }?;

        let full_rotations = amt / 100;
        zero_count += full_rotations;

        let true_amt = (amt % 100) * dir;

        pos += true_amt;

        if starting_pos != 0 && (pos <= 0 || pos >= 100) {
            zero_count += 1;
        }

        if pos < 0 {
            pos += 100;
        }
        else {
            pos %= 100;
        }
    }

    Ok(zero_count)
}

fn main() -> Result<()> {
    let input = fs::read_to_string(absolute("../input_files/day01.input.txt")?)?;
    let lines: Vec<&str> = input.split_terminator('\n').collect();

    println!("{}", part1(lines.clone())?);
    println!("{}", part2(lines)?);

    Ok(())
}
