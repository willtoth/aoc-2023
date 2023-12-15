use std::{fmt::Debug, ops::Rem, vec};

use std::ops::Range;

use crate::geometry::{BoundingBox, Point, Rectangle};

pub struct GridRowIterator<'a, T: Copy + Clone> {
    grid: &'a Grid2d<T>,
    row: i64,
    col: i64,
}

impl<'a, T: Copy + Clone> Iterator for GridRowIterator<'a, T> {
    type Item = (Point<i64>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.index(self.col, self.row).ok();
        self.col += 1;

        if val.is_none() {
            return None;
        }

        Some((Point::new(self.col, self.row), val.unwrap()))
    }
}

pub struct GridColIterator<'a, T: Copy + Clone> {
    grid: &'a Grid2d<T>,
    row: i64,
    col: i64,
}

// impl<'a, T: Copy + Clone> DoubleEndedIterator for GridColIterator<'a, T> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

impl<'a, T: Copy + Clone> Iterator for GridColIterator<'a, T> {
    type Item = (Point<i64>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.index(self.col, self.row).ok();
        self.row += 1;

        if val.is_none() {
            return None;
        }

        Some((Point::new(self.col, self.row), val.unwrap()))
    }
}

pub struct GridIterator<'a, T: Copy + Clone> {
    grid: &'a Grid2d<T>,
    x: i64,
    y: i64,
}

impl<'a, T: Copy + Clone> Iterator for GridIterator<'a, T> {
    type Item = (Point<i64>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.index(self.x, self.y).ok();

        let current_x = self.x;
        let current_y = self.y;

        self.x += 1;

        if !self.grid.in_bounds(&Point {
            x: self.x,
            y: self.y,
        }) {
            self.x = self.grid.coord_top_left.x;
            self.y += 1;
        }

        if val.is_none() {
            return None;
        }

        Some((Point::new(current_x, current_y), val.unwrap()))
    }
}

#[derive(Clone)]
pub struct Grid2d<T: Copy + Clone> {
    grid: Vec<Vec<T>>,
    coord_top_left: Point<i64>,
    pub default: T,
    bounds: Option<Rectangle<i64>>,
}

impl<T: Copy + Clone + PartialEq> Grid2d<T> {
    pub fn grid_eq(&self, other: &Grid2d<T>) -> bool {
        self.grid == other.grid
    }
}

impl<T: Copy + Clone> Grid2d<T> {
    pub fn new(default: T) -> Grid2d<T> {
        Grid2d {
            grid: vec![vec![default; 1]; 1],
            coord_top_left: Point::new(0, 0),
            default,
            bounds: None,
        }
    }

    pub fn rotate_ccw(&self) -> Grid2d<T> {
        let rows = self.grid[0].len();
        let cols = self.grid.len();
        let mut result = Grid2d {
            grid: vec![vec![self.default; rows]; cols],
            coord_top_left: Point::new(0, 0), // TODO:Fix this?
            default: self.default,
            bounds: None, // TODO: Fix this
        };

        for (p, value) in self.iter() {
            result.set_or_insert(p.y, cols as i64 - p.x, *value);
        }

        result
    }

    pub fn rotate_cw(&self) -> Grid2d<T> {
        let rows = (self.max_x() - 1) as usize; //TODO: this whole lib is broken...
        let cols = (self.max_y() - 1) as usize;
        let mut result = Grid2d {
            grid: vec![vec![self.default; rows]; cols],
            coord_top_left: Point::new(0, 0), // TODO:Fix this?
            default: self.default,
            bounds: None, // TODO: Fix this
        };

        for (p, value) in self.iter() {
            result.set_or_insert(rows as i64 - p.y, p.x, *value);
        }

        result
    }

    pub fn index(&self, x: i64, y: i64) -> Result<&T, ()> {
        if !self.in_bounds(&Point::new(x, y)) {
            return Err(());
        }
        Ok(&self.grid[(y - self.coord_top_left.y) as usize][(x - self.coord_top_left.x) as usize])
    }

    pub fn index_mut(&mut self, x: i64, y: i64) -> &mut T {
        &mut self.grid[(y - self.coord_top_left.y) as usize][(x - self.coord_top_left.x) as usize]
    }

    pub fn max_y(&self) -> i64 {
        self.grid.len() as i64 + self.coord_top_left.y
    }

    pub fn max_x(&self) -> i64 {
        self.grid[0].len() as i64 + self.coord_top_left.x
    }

    pub fn min_y(&self) -> i64 {
        self.coord_top_left.y
    }

    pub fn min_x(&self) -> i64 {
        self.coord_top_left.x
    }

    pub fn range_y(&self) -> Range<i64> {
        self.min_y()..self.max_y()
    }

    pub fn range_x(&self) -> Range<i64> {
        self.min_x()..self.max_x()
    }

    pub fn iter_fixed_y(&self, y: i64) -> GridRowIterator<T> {
        GridRowIterator {
            grid: self,
            row: y,
            col: self.coord_top_left.x,
        }
    }

    pub fn iter_fixed_x(&self, x: i64) -> GridColIterator<T> {
        GridColIterator {
            grid: self,
            row: self.coord_top_left.y,
            col: x,
        }
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            x: self.coord_top_left.x,
            y: self.coord_top_left.y,
        }
    }

    fn row_from_y(&self, y: i64) -> usize {
        (y - self.coord_top_left.y) as usize
    }
    fn col_from_x(&self, x: i64) -> usize {
        (x - self.coord_top_left.x) as usize
    }

    pub fn insert_row_after(&mut self, y: i64) {
        self.grid
            .insert(self.row_from_y(y), vec![self.default; self.grid[0].len()]);
    }

    pub fn insert_col_after(&mut self, x: i64) {
        let col = self.col_from_x(x);
        for row in 0..self.grid.len() {
            self.grid[row].insert(col, self.default);
        }
    }

    pub fn set_or_insert(&mut self, x: i64, y: i64, value: T) {
        let p = Point::new(x, y);
        let bounds = self.bounds();

        if let Some(a) = self.bounds {
            if !a.in_bounds(&Point::new(x, y)) {
                return;
            }
        }

        if !self.in_bounds(&p) {
            // Add columns before
            if x < bounds.tl.x {
                let num_to_add = (bounds.tl.x - x) as usize;

                for i in 0..bounds.height() {
                    let mut new_vec = vec![self.default; num_to_add];
                    new_vec.append(&mut self.grid[i as usize]);
                    self.grid[i as usize] = new_vec;
                }

                // Move first point
                self.coord_top_left.x = x;
            }
            let bounds = self.bounds();

            // Add rows before
            if y < bounds.tl.y {
                let num_to_add = (bounds.tl.y - y) as usize;
                let mut new_row = vec![vec![self.default; bounds.width() as usize]; num_to_add];
                new_row.append(&mut self.grid);

                self.grid = new_row;

                // Move first point
                self.coord_top_left.y = y;
            }
            let bounds = self.bounds();

            // Add columns after
            if x >= bounds.br.x {
                let num_to_add = (x - bounds.br.x + 1) as usize;

                for i in 0..bounds.height() {
                    for _ in 0..num_to_add {
                        self.grid[i as usize].push(self.default.clone());
                    }
                }
            }
            let bounds = self.bounds();

            // Add rows after
            if y >= bounds.br.y {
                let num_to_add = (y - bounds.br.y + 1) as usize;

                for _ in 0..num_to_add {
                    self.grid.push(vec![self.default; bounds.width() as usize]);
                }
            }
        }

        self.grid[(y - self.coord_top_left.y) as usize][(x - self.coord_top_left.x) as usize] =
            value
    }

    pub fn clear(&mut self) {
        for row in self.grid.iter_mut() {
            for ch in row.iter_mut() {
                *ch = self.default;
            }
        }
    }
}

impl<T: Clone + Copy> Grid2d<T> {
    pub fn new_from_vec(grid: &Vec<Vec<T>>, default: T) -> Grid2d<T> {
        Grid2d {
            grid: grid.clone(),
            coord_top_left: Point::new(0, 0),
            default,
            bounds: None,
        }
    }

    pub fn new_with_size(width: usize, height: usize, start_val: T) -> Grid2d<T> {
        Grid2d {
            grid: vec![vec![start_val; width]; height],
            coord_top_left: Point::new(0, 0),
            default: start_val,
            bounds: None,
        }
    }

    pub fn new_with_coordinates(coords: Rectangle<i64>, start_val: T) -> Grid2d<T> {
        Grid2d {
            grid: vec![vec![start_val; coords.width() as usize]; coords.height() as usize],
            coord_top_left: coords.tl,
            default: start_val,
            bounds: None,
        }
    }

    pub fn set_max_bounds(&mut self, bounds: Rectangle<i64>) {
        self.bounds = Some(bounds);
    }
}

impl<T: Copy> BoundingBox<i64> for Grid2d<T> {
    fn bounds(&self) -> Rectangle<i64> {
        let x_len = if self.grid.len() == 0 {
            0
        } else {
            self.grid[0].len()
        } as i64;
        Rectangle::new(
            self.coord_top_left,
            self.coord_top_left + Point::new(x_len, self.grid.len() as i64),
        )
    }
}

fn get_digit(x: i64, digit: usize) -> char {
    let arr = x.abs().to_string().chars().rev().collect::<Vec<char>>();

    if digit >= arr.len() {
        return '0';
    } else {
        return arr[digit];
    }
}

impl<T: ToString + Copy> Debug for Grid2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounds();
        let width = ((bounds.height() as f64).log(10.0).floor() as usize) + 3;
        let pad_height = ((bounds.width() as f64).log(10.0).floor() as usize) + 1;

        if f.alternate() {
            // Print header
            for i in (0..pad_height + 1).rev() {
                print!("{:width$}", " ", width = width);
                for j in bounds.tl.x..bounds.br.x {
                    if j.rem(5) == 0 {
                        let digit = get_digit(j, i);
                        if i == pad_height && j.is_negative() {
                            print!("-");
                        } else if digit == '0' && i != 0 {
                            print!(" ");
                        } else {
                            print!("{digit}");
                        }
                    } else {
                        print!(" ");
                    }
                }
                println!("");
            }
        }

        for y in bounds.tl.y..bounds.br.y {
            if f.alternate() {
                print!("{:<width$}", y, width = width);
            }

            for x in bounds.tl.x..bounds.br.x {
                print!("{}", self.index(x, y).unwrap().to_string());
            }
            println!("");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_creation() {
        let grid = Grid2d::new_with_size(10, 10, '.');
        assert_eq!(grid.grid[9][9], '.');

        let r = Rectangle::new(Point { x: 5, y: 5 }, Point { x: -5, y: -5 });

        let grid = Grid2d::new_with_coordinates(r, '!');
        assert_eq!(grid.grid[9][9], '!');
    }

    #[test]
    fn set_or_insert() {
        let mut grid = Grid2d::new_with_size(10, 10, '.');
        assert_eq!(*grid.index(5, 5).unwrap(), '.');
        grid.set_or_insert(5, 5, '#');
        assert_eq!(*grid.index(5, 5).unwrap(), '#');

        grid.set_or_insert(10, 10, 'w');
        assert_eq!(*grid.index(10, 10).unwrap(), 'w');

        grid.set_or_insert(15, 15, '8');
        assert_eq!(*grid.index(15, 15).unwrap(), '8');
        assert_eq!(*grid.index(12, 12).unwrap(), '.');

        grid.set_or_insert(-8, -3, 'M');
        assert_eq!(*grid.index(-8, -3).unwrap(), 'M');
        assert_eq!(*grid.index(-2, -1).unwrap(), '.');
        assert_eq!(grid.coord_top_left, Point::new(-8, -3));
        assert_eq!(grid.index(-9, -4), Err(()));
    }
}
