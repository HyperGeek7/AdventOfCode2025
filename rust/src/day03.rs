use std::{cmp::Ordering, fs, path::absolute};

use anyhow::Result;

fn compare_elements(a: &(usize, u32), b: &(usize, u32)) -> Ordering {
    // The idea here is sort of a half-reversed sort.
    // We want higher numbers to end up at the front of the result,
    // but we also want matching digits to end up sorted in string order.
    let a_val: u32 = (a.1 * 1000) + (100 - (a.0 as u32));
    let b_val = (b.1 * 1000) + (100 - (b.0 as u32));

    b_val.cmp(&a_val)
}

fn make_sorted_line(line: &&str) -> Vec<(usize, u32)> {
    // Given a &str, run it through char_indices to get the elements
    // we'll operate on for the rest of the script, convert the chars
    // to u32s, and run them through the sort function.
    // This gives us prepared input we'll need for both parts.

    let mut sorted_values: Vec<(usize, u32)> = line
        .char_indices()
        .map(|(pos, char)| (pos, char.to_digit(10).unwrap()))
        .collect();

    sorted_values
        .sort_by(
            compare_elements
        );

    sorted_values
}

fn highest_available(sorted_values: &Vec<(usize, u32)>, min_index: i32, max_index: usize) -> (usize, u32) {
    // As we assemble our number, we consistently want to pull the highest number
    // that matches the following criteria:
    // 1. It must come after the last number we selected
    // 2. It must have enough digits after it to finish filling out the result
    // This is fairly simple to write into a filter function
    // and then just pop off the highest value that remains.

    // min_index should really be an Option<usize> that just omits
    // the condition entirely if it's None, but I couldn't think of
    // how to do it this morning, so we get this weird casting nonsense
    // instead.

    *(sorted_values
        .iter()
        .filter(|(pos, _char)| *pos as i32 > min_index && *pos <= max_index)
        .nth(0)
        .unwrap()
    )
}

fn part1(lines: &Vec<&str>) -> Result<u32> {
    let mut result = 0;

    for line in lines {
        //println!("Processing line {}", line);
        let mut this_line_value = 0;

        let sorted_values = make_sorted_line(line);

        let max_value: u32;
        let max_index: usize;

        // We want to use the highest value character as our tens place,
        // but that only works if it's not the last character in the string.
        if sorted_values[0].0 < line.len() - 1 {
            (max_index, max_value) = sorted_values[0];
        }
        else {
            (max_index, max_value) = sorted_values[1];
        }

        this_line_value += max_value * 10;

        let next_highest = sorted_values
            .iter()
            .filter(|(pos, _char)| *pos > max_index)
            .nth(0)
            .unwrap()
            .1;
        this_line_value += next_highest;

        //println!("This line's value: {}", this_line_value);
        result += this_line_value;
    }

    Ok(result)
}

fn part2(lines: &Vec<&str>) -> Result<u64> {
    let mut result = 0;

    for line in lines {
        let mut digit_count = 12;

        //println!("Processing line {}", line);
        let mut this_line_value = 0;
        let mut last_digit_index = -1;

        let sorted_values = make_sorted_line(line);

        while digit_count > 0 {
            let max_index = line.len() - digit_count;
            let this_digit;
            let next_highest = highest_available(&sorted_values, last_digit_index, max_index);
            (last_digit_index, this_digit) = (next_highest.0 as i32, next_highest.1 as u64);
            digit_count -= 1;
            this_line_value += this_digit * (10_u64.pow(digit_count as u32));
        }

        //println!("This line's value: {}", this_line_value);
        result += this_line_value;
    }

    Ok(result)
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day03.input.txt")?)?;
    let input_lines: Vec<&str> = input_string
        .lines()
        .collect();

    println!("{}", part1(&input_lines)?);
    println!("{}", part2(&input_lines)?);

    Ok(())
}