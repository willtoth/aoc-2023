#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::fs;

use aoc_2023::geometry::{Grid2d, Point};

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

    // Find in rows of new space
    let mut y_to_add: Vec<i64> = vec![];
    for y in grid.range_y() {
        if grid.iter_fixed_y(y).all(|(_, ch)| *ch == '.') {
            y_to_add.push(y);
        }
    }

    // Find in cols of new space
    let mut x_to_add: Vec<i64> = vec![];
    for x in grid.range_x() {
        if grid.iter_fixed_x(x).all(|(_, ch)| *ch == '.') {
            x_to_add.push(x);
        }
    }

    // Find locations of stars
    let mut points = grid
        .iter()
        .filter_map(|(p, ch)| if *ch == '#' { Some(p) } else { None })
        .collect::<Vec<Point<i64>>>();

    for p in points.iter_mut() {
        let y_cnt = y_to_add.iter().filter(|y| **y < p.y).count()
            * if cfg!(feature = "part2") { 999999 } else { 1 };
        let x_cnt = x_to_add.iter().filter(|x| **x < p.x).count()
            * if cfg!(feature = "part2") { 999999 } else { 1 };
        p.x += x_cnt as i64;
        p.y += y_cnt as i64;
    }

    // Calculate their distances and sum them
    let mut result = 0;
    for (idx, p) in points.iter().enumerate() {
        result += points
            .iter()
            .skip(idx)
            .map(|inner_p| p.manhattan_distance(inner_p))
            .sum::<i64>();
    }

    println!("Result: {:?}", result);
}
