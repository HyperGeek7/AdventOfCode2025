use itertools::iproduct;
use std::{collections::HashMap, fs, path::absolute};

use anyhow::Result;

fn prep_map(lines: &Vec<&str>) -> HashMap<(i32, i32), char> {
    let mut floor_map: HashMap<(i32, i32), char> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.char_indices() {
            // This could really *just* contain the locations of the
            // "@" characters to save some time and memory,
            // but I didn't realize that until I was basically done.
            // Alas.
            floor_map.insert((x as i32, y as i32), char);
        }
    }

    floor_map
}

fn part1(input_lines: &Vec<&str>) -> Result<usize> {
    let floor_map = prep_map(input_lines);

    // Oh look, another functional programming hellscape!
    // ...Listen, I just find these fun to write. They're definitely
    // not the *best* way to handle everything.

    // On the other hand, this particular puzzle lends itself to this sort of approach:
    // We have a grid of characters. We want to transform that into a grid of numbers
    // (how many rolls each roll is touching), and then count how many elements of *that*
    // grid are less than 4. So, easy enough to do.

    let result = floor_map
        // Iterate over all the key-value pairs
        .iter()
        // Discard any spaces that aren't occupied
        .filter(|((_, _), char)| **char == '@')
        // Count how many rolls each roll is touching, which is its own (somewhat involved) function.
        .map(|((x, y), _)| {
            // Get the coordinates of all the surrounding "tiles" by way of the cartesian product.
            iproduct!((x - 1)..(x + 2), (y - 1)..(y + 2))
                // The cartesian product also includes the middle tile itself, so discard that.
                .filter(|(offset_x, offset_y)| offset_x != x || offset_y != y)
                // Discard any of the surrounding tiles that don't contain another roll
                .filter_map(|(offset_x, offset_y)| {
                    match floor_map.get(&(offset_x, offset_y)).unwrap_or(&'.') {
                        '@' => Some(1),
                        _ => None,
                    }
                })
                // And count how many we have left
                .count()
        })
        // So that terrifying map() call has transformed our original grid of characters into a
        // sequence of numbers that indicates how many rolls each tile is touching.
        // We've lost the actual grid component along the way, but that doesn't matter for
        // this particular puzzle.
        // That done, we just discard any tile with more than 3 occupied neighbors...
        .filter(|occupied_slots| *occupied_slots < 4)
        // And then count how many elements we have left.
        .count();

    // See? Nothing simpler!

    Ok(result)
}

fn part2(input_lines: &Vec<&str>) -> Result<usize> {
    let mut floor_map = prep_map(input_lines);
    let mut result = 0;

    // This is basically just part 1 in a loop, so see that function for
    // how all the heavy lifting works.

    // The key difference this time is in what we're returning from
    // the functional programming hellscape: instead of just the count,
    // we're also getting back the coordinates of the point.
    // Once we note how many of these points there are, we also
    // remove them from the map, then go back and run the process again.
    // Once we have a run where nothing comes back, we're done.

    loop {
        let these_moves: Vec<(i32, i32, usize)> = floor_map
            .iter()
            .filter(|((_, _), char)| **char == '@')
            .map(|((x, y), _)| {
                (
                    *x,
                    *y,
                    iproduct!((x - 1)..(x + 2), (y - 1)..(y + 2))
                        .filter(|(offset_x, offset_y)| offset_x != x || offset_y != y)
                        .filter_map(|(offset_x, offset_y)| {
                            match floor_map.get(&(offset_x, offset_y)).unwrap_or(&'.') {
                                '@' => Some(1),
                                _ => None,
                            }
                        })
                        .count(),
                )
            })
            .filter(|(_, _, occupied_slots)| *occupied_slots < 4)
            .collect();

        if these_moves.is_empty() {
            break;
        }

        result += these_moves.len();

        for (x, y, _) in these_moves {
            floor_map.remove(&(x, y));
        }
    }

    Ok(result)
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(absolute("../input_files/day04.input.txt")?)?;
    let input_lines: Vec<&str> = input_string.lines().collect();

    println!("{}", part1(&input_lines)?);
    println!("{}", part2(&input_lines)?);

    Ok(())
}
