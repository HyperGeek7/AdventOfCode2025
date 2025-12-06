use std::{fs, path::absolute};

use anyhow::{Result, anyhow};
use itertools::Itertools;

fn parse_input(input_lines: &Vec<&str>) -> Vec<Vec<String>> {
    let column_count = input_lines
        .first()
        .expect("No input given?!")
        .split_whitespace()
        .collect_vec()
        .len();

    let mut result = vec![Vec::new(); column_count];

    for line in input_lines {
        line.split_whitespace()
            .enumerate()
            .for_each(|(i, str)| result[i].push(str.into()));
    }

    result
}

fn part1(input_lines: &Vec<&str>) -> Result<u64> {
    let input_columns = parse_input(input_lines);
    let mut result: u64 = 0;

    for mut column in input_columns {
        let operation = column.pop().expect("Got an empty input column");

        let operands = column
            .into_iter()
            .map(|x| x.parse::<u64>().expect("Unable to parse operand"));

        if operation == "+" {
            result += operands.sum::<u64>();
        } else if operation == "*" {
            result += operands.product::<u64>()
        } else {
            return Err(anyhow!("Got something that very was not an operator!"));
        }
    }

    Ok(result)
}

fn vertical_parse(input_lines: &Vec<&str>) -> (Vec<Vec<u64>>, Vec<String>) {
    // Here's some horrific shenaniganry! A sane man would have written actual loops,
    // but we're stuck with me.

    // First thing: let's get a range over every valid index into the input strings.
    // The input strings are always the same length, so this is safe, but I'll do some
    // double-checking later anyways. For laughs.
    let operands = (0..input_lines
        .first()
        .expect("Some input would be nice.")
        .len())
        .map(|i| {
            // Now, we're iterating over i as every valid index in every line,
            // so to read a single column, we just fetch the i-th character in each line.
            input_lines
                .iter()
                .map(|&line| line.chars().nth(i).unwrap_or(' '))
                // It's easier to just get the operators after the fact, so discard any character that isn't
                // a valid base 10 digit. This also effectively trims the whitespace.
                .filter_map(|x| match x.is_ascii_digit() {
                    true => Some(x.to_string()),
                    false => None,
                })
                // Join the individual digit strings into the full number.
                .join("")
        })
        .collect_vec()
        // So we now have a vector of strings containing each of the columns.
        // There will be at least one blank column in between each "group" of columns,
        // so we can use split() to chunk them out.
        .split(|x| x.is_empty())
        // We now have a vector of slices, which is a little awkward to work with,
        // but each element of those slices is a string containing a valid number.
        // We can use a nested map call to parse each of them into a numeric type,
        // convert the slice to a Vec for good measure, and then collect the whole mess.
        .map(|x| x.iter().map(|y| y.parse::<u64>().unwrap()).collect_vec())
        .collect_vec();

    // Operators are much simpler: grab the last line of input
    // and separate on whitespace.
    // I'm also converting to String here just because I don't
    // fully understand lifetimes and this sidesteps that,
    // but I think a more experienced programmer could skip that.
    let operators = input_lines
        .last()
        .expect("The impossible is real")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect_vec();
    (operands, operators)
}

fn part2(input_lines: &Vec<&str>) -> Result<u64> {
    let (operand_columns, operators) = vertical_parse(input_lines);

    let result = operators
        .into_iter()
        .enumerate()
        // This is what I really should have written in part 1.
        // I'm leaving that one as is, just for the sake of comparison.
        .map(|(i, operator)| match operator.as_str() {
            "+" => Ok(operand_columns
                .get(i)
                .expect("Mismatched number of columns and operators!")
                .iter()
                .sum::<u64>()),
            "*" => Ok(operand_columns
                .get(i)
                .expect("Mismatched number of columns and operators!")
                .iter()
                .product::<u64>()),
            _ => Err(anyhow!("Got something that wasn't an operator!")),
        }.unwrap())
        .sum();

    Ok(result)
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day06.input.txt")?)?;
    let input_lines: Vec<&str> = input_string.lines().collect();

    println!("{}", part1(&input_lines)?);
    println!("{}", part2(&input_lines)?);

    Ok(())
}
