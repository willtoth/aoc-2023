#![feature(ascii_char)]
#![feature(iter_order_by)]

use std::fs;

use aoc_2023::geometry::{BoundingBox, Grid2d, Point};

fn find_candidates(grid: &Grid2d<char>) -> (Vec<usize>, Vec<usize>) {
    let mut rows: Vec<usize> = vec![];
    let mut cols: Vec<usize> = vec![];
    for x_col in 0..(grid.max_x() - 1) {
        let col = grid.iter_fixed_x(x_col);
        let next_col = grid.iter_fixed_x(x_col + 1);

        if col.eq_by(next_col, |x, y| x.1 == y.1) {
            cols.push(x_col as usize);
        }
    }

    for y_row in 0..(grid.max_y() - 1) {
        let row = grid.iter_fixed_y(y_row);
        let next_row = grid.iter_fixed_y(y_row + 1);

        if row.eq_by(next_row, |x, y| x.1 == y.1) {
            rows.push(y_row as usize);
        }
    }

    (rows, cols)
}

fn check_row_candidate(grid: &Grid2d<char>, row: usize) -> Option<i64> {
    let left = row as i64;
    let right = (row + 1) as i64;
    let mut cnt = 0;
    let mut differences = 0;

    while grid.in_bounds(&Point::new(0, left - cnt)) && grid.in_bounds(&Point::new(0, right + cnt))
    {
        let mut iterators = (
            grid.iter_fixed_y(left - cnt),
            grid.iter_fixed_y(right + cnt),
        );
        cnt += 1;

        while let (Some(liter), Some(riter)) = (iterators.0.next(), iterators.1.next()) {
            if *liter.1 != *riter.1 {
                differences += 1;
            }

            if differences > 1 {
                return None;
            }
        }

        // if !iterators.0.eq_by(iterators.1, |x, y| x.1 == y.1) {
        //     return None;
        // }
    }

    if cnt == 0 || differences == 0 {
        return None;
    }
    Some(cnt)
}

fn check_col_candidate(grid: &Grid2d<char>, col: usize) -> Option<i64> {
    let left = col as i64;
    let right = (col + 1) as i64;
    let mut cnt = 0;
    let mut differences = 0;

    while grid.in_bounds(&Point::new(left - cnt, 0)) && grid.in_bounds(&Point::new(right + cnt, 0))
    {
        let mut iterators = (
            grid.iter_fixed_x(left - cnt),
            grid.iter_fixed_x(right + cnt),
        );
        cnt += 1;

        while let (Some(liter), Some(riter)) = (iterators.0.next(), iterators.1.next()) {
            if *liter.1 != *riter.1 {
                differences += 1;
            }

            if differences > 1 {
                return None;
            }
        }

        // if !iterators.0.eq_by(iterators.1, |x, y| x.1 == y.1) {
        //     return None;
        // }
    }

    if cnt == 0 || differences == 0 {
        return None;
    }
    Some(cnt)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let grids = &input
        .split("\n\n")
        .map(|grid| {
            let grid = grid
                .split("\n")
                .map(|row| {
                    row.as_ascii()
                        .unwrap()
                        .iter()
                        .map(|ch| ch.to_char())
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();

            Grid2d::new_from_vec(&grid, '.')
        })
        .collect::<Vec<Grid2d<char>>>();

    let result = grids
        .iter()
        .map(|grid| {
            //let (rows, cols) = find_candidates(grid);
            let (rows, cols) = (
                (0..grid.max_y() as usize)
                    .into_iter()
                    .collect::<Vec<usize>>(),
                (0..grid.max_x() as usize)
                    .into_iter()
                    .collect::<Vec<usize>>(),
            );

            let row_val = rows
                .into_iter()
                .filter(|row| check_row_candidate(grid, *row).is_some())
                .map(|x| (x + 1) * 100)
                .nth(0);

            let col_val = cols
                .into_iter()
                .filter(|col| check_col_candidate(grid, *col).is_some())
                .map(|x| (x + 1))
                .nth(0);
            row_val.unwrap_or(col_val.unwrap_or_default())
        })
        .sum::<usize>();

    println!("Result: {:?}", result);
}
