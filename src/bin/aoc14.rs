#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::fs;

use aoc_2023::geometry::Grid2d;

fn tilt(grid: &Grid2d<char>) -> Grid2d<char> {
    let mut result = grid.clone();
    let mut location_stack: Vec<usize> = vec![];
    result.clear();

    for x in grid.range_x() {
        location_stack.clear();
        for (y, (_, value)) in grid.iter_fixed_x(x).enumerate() {
            match value {
                '.' => location_stack.push(y),
                'O' => {
                    if let Some(location) = location_stack.first() {
                        // Swap this space with another
                        result.set_or_insert(x as i64, *location as i64, *value);
                        location_stack.push(y);
                        location_stack.remove(0);
                    } else {
                        result.set_or_insert(x as i64, y as i64, *value);
                    }
                }
                '#' => {
                    location_stack.clear();
                    result.set_or_insert(x as i64, y as i64, *value);
                }
                _ => panic!(),
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let grid = &input
        .lines()
        .map(|row| {
            row.as_ascii()
                .unwrap()
                .iter()
                .map(|ch| ch.to_char())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    // Creat the grid
    let grid = Grid2d::new_from_vec(&grid, '.');
    let part1 = tilt(&grid);
    let max_val = grid.max_y();

    let result = part1
        .iter()
        .filter_map(|(p, value)| {
            if *value == 'O' {
                Some(max_val - p.y)
            } else {
                None
            }
        })
        .sum::<i64>();

    println!("Part 1: {:?}", result);

    // Part 2
    // one rotation
    let mut result = grid.clone();

    // Run a bunch until the puzzle is repeating
    for _ in 0..999 {
        for _ in 0..4 {
            result = tilt(&result);
            result = result.rotate_cw();
        }
    }

    // Find the mod that its rotating at
    let result2 = result.clone();
    let mut modulus = 0;
    for i in 0..10000 {
        for _ in 0..4 {
            result = tilt(&result);
            result = result.rotate_cw();
        }
        if result2.grid_eq(&result) {
            modulus = i + 1;
            break;
        }
    }

    // Rotate congruent to 1000000000th run
    for _ in 0..((1000000000 - 999) % modulus) {
        for _ in 0..4 {
            result = tilt(&result);
            result = result.rotate_cw();
        }
    }

    let part2 = result
        .iter()
        .filter_map(|(p, value)| {
            if *value == 'O' {
                Some(max_val - p.y)
            } else {
                None
            }
        })
        .sum::<i64>();

    println!("Part 2: {:?}", part2);
}
