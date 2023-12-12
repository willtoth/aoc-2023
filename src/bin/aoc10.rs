// If you're reading this, sorry in advance. This one was hacky...

#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::fs;

use num::ToPrimitive;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn rotate_cw(&self) -> Direction {
        match *self {
            Self::NORTH => Self::EAST,
            Self::EAST => Self::SOUTH,
            Self::SOUTH => Self::WEST,
            Self::WEST => Self::NORTH,
        }
    }

    fn rotate_ccw(&self) -> Direction {
        match *self {
            Self::NORTH => Self::WEST,
            Self::EAST => Self::NORTH,
            Self::SOUTH => Self::EAST,
            Self::WEST => Self::SOUTH,
        }
    }
}

fn direction_update(current_point: &Point, next_point: &Point) -> Direction {
    // Pos = NORTH, Neg = SOUTH
    let ver_dir = current_point.0 - next_point.0;
    // Pos = WEST, Neg = EAST
    let hor_dir = current_point.1 - next_point.1;

    match (ver_dir, hor_dir) {
        (1, _) => Direction::NORTH,
        (-1, _) => Direction::SOUTH,
        (_, 1) => Direction::WEST,
        (_, -1) => Direction::EAST,
        _ => panic!(),
    }
}

fn point_in_direction(direction: &Direction, p: &Point) -> Point {
    match direction {
        Direction::NORTH => Point(p.0 - 1, p.1),
        Direction::EAST => Point(p.0, p.1 + 1),
        Direction::SOUTH => Point(p.0 + 1, p.1),
        Direction::WEST => Point(p.0, p.1 - 1),
    }
}

// (ROW, COL)
#[derive(Debug, Clone, Copy)]
struct Point(i32, i32);

#[derive(Debug, Clone, Copy)]
struct Node {
    a: Point,
    b: Point,
    me: Point,
    start: bool,
    score: i32,
}

fn in_bounds(map: &Vec<Vec<i32>>, p: &Point) -> bool {
    p.0 >= 0 && (p.0 < map.len() as i32) && p.1 >= 0 && p.1 < (map[0].len() as i32)
}

fn node_from_point(map: &Vec<Vec<Option<Node>>>, p: &Point) -> Option<Node> {
    map[p.0 as usize][p.1 as usize].clone()
}

fn get_score(map: &Vec<Vec<i32>>, p: &Point) -> i32 {
    map[p.0 as usize][p.1 as usize]
}

fn set_score(map: &mut Vec<Vec<i32>>, p: &Point, s: i32) {
    map[p.0 as usize][p.1 as usize] = s
}

fn draw_map(map: &Vec<Vec<i32>>) {
    for row in map {
        for col in row {
            let to_print = match *col {
                i32::MAX => ".",
                -1 => "I",
                -10 => "^",
                -11 => "V",
                -12 => ">",
                -13 => "<",
                _ => "X",
            };
            print!("{to_print}");
        }
        println!("");
    }
    println!("");
}

fn flood(map: &mut Vec<Vec<i32>>, p: &Point) {
    // 4 connected
    let points = [
        Point(p.0, p.1 - 1),
        Point(p.0 - 1, p.1),
        Point(p.0, p.1 + 1),
        Point(p.0 + 1, p.1),
    ];
    let points = points
        .iter()
        .filter(|x| in_bounds(map, x))
        .collect::<Vec<&Point>>();

    points.iter().for_each(|x| {
        if map[x.0 as usize][x.1 as usize] == i32::MAX {
            map[x.0 as usize][x.1 as usize] = -1;
        }
        //flood(map, *x);
    });
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let cols = input.lines().count();
    let rows = input.lines().nth(0).unwrap().len();

    // Part 1
    let input = input
        .lines()
        .enumerate()
        .map(|(row, row_str)| {
            row_str
                .as_ascii()
                .unwrap()
                .into_iter()
                .enumerate()
                .map(|(col, col_str)| match col_str {
                    std::ascii::Char::VerticalLine => Some(Node {
                        a: Point(row as i32 + 1, col as i32),
                        b: Point(row as i32 - 1, col as i32),
                        me: Point(row as i32, col as i32),
                        start: false,
                        score: 0,
                    }),
                    std::ascii::Char::HyphenMinus => Some(Node {
                        a: Point(row as i32, col as i32 + 1),
                        b: Point(row as i32, col as i32 - 1),
                        me: Point(row as i32, col as i32),
                        start: false,
                        score: 0,
                    }),
                    std::ascii::Char::CapitalL => Some(Node {
                        a: Point(row as i32 - 1, col as i32),
                        b: Point(row as i32, col as i32 + 1),
                        me: Point(row as i32, col as i32),
                        start: false,
                        score: 0,
                    }),
                    std::ascii::Char::CapitalJ => Some(Node {
                        a: Point(row as i32, col as i32 - 1),
                        b: Point(row as i32 - 1, col as i32),
                        me: Point(row as i32, col as i32),
                        start: false,
                        score: 0,
                    }),
                    std::ascii::Char::Digit7 => Some(Node {
                        a: Point(row as i32, col as i32 - 1),
                        b: Point(row as i32 + 1, col as i32),
                        me: Point(row as i32, col as i32),
                        start: false,
                        score: 0,
                    }),
                    std::ascii::Char::CapitalF => Some(Node {
                        a: Point(row as i32, col as i32 + 1),
                        b: Point(row as i32 + 1, col as i32),
                        me: Point(row as i32, col as i32),
                        start: false,
                        score: 0,
                    }),
                    std::ascii::Char::CapitalS => {
                        // Lazy, just hard code this to puzzel...
                        // // input
                        let a = Point(row as i32, col as i32 - 1);
                        let b = Point(row as i32, col as i32 + 1);
                        Some(Node {
                            a,
                            b,
                            me: Point(row as i32, col as i32),
                            start: true,
                            score: 0,
                        })
                    }
                    _ => None,
                })
                .collect::<Vec<Option<Node>>>()
        })
        .collect::<Vec<Vec<Option<Node>>>>();

    let start = input
        .iter()
        .flatten()
        .find(|x| x.is_some_and(|y| y.start))
        .unwrap()
        .unwrap();

    let mut map = vec![vec![i32::MAX; rows]; cols];
    let mut pol_map = vec![vec![i32::MAX; rows]; cols];
    let mut current_direction = Direction::EAST;

    let mut point = Some(start);
    let mut prev_point = point;
    set_score(&mut map, &start.me, 0);
    let mut score = 0;

    // Work clockwise, 'inner' is to the 'right'
    while let Some(x) = point {
        //println!("{:?}", point);
        //draw_map(&map);
        score = score + 1;
        let next_point = if get_score(&map, &x.a) > score {
            &x.a
        } else if get_score(&map, &x.b) > score {
            &x.b
        } else {
            set_score(&mut pol_map, &x.me, (10) * -1);
            break;
        };

        let pind = &point_in_direction(&current_direction.rotate_cw(), &x.me);
        current_direction = direction_update(&x.me, next_point);

        set_score(&mut map, next_point, score);

        if get_score(&map, pind) == i32::MAX {
            set_score(&mut pol_map, pind, -1);
        }

        let pind = &point_in_direction(&current_direction.rotate_cw(), &x.me);

        set_score(&mut map, next_point, score);

        if get_score(&map, pind) == i32::MAX {
            set_score(&mut pol_map, pind, -1);
        }

        point = node_from_point(&input, next_point);
        set_score(
            &mut pol_map,
            &x.me,
            (direction_update(&x.me, next_point) as i32 + 10) * -1,
        );
    }
    println!("==========");
    let mut score = 0;
    let mut point = Some(start);

    while let Some(x) = point {
        //println!("{:?}", point);
        //draw_map(&map);
        score = score + 1;
        if get_score(&map, &x.b) > score {
            set_score(&mut map, &x.b, score);
            point = node_from_point(&input, &x.b);
        } else if get_score(&map, &x.a) > score {
            set_score(&mut map, &x.a, score);
            point = node_from_point(&input, &x.a);
        } else {
            point = None;
        }
    }

    draw_map(&map);

    // Part 1
    println!(
        "Part 1: {:?}",
        map.iter().flatten().filter(|x| **x != i32::MAX).max()
    );

    // Flood each node that is empty. If it enounters an edge its no in the loop.
    for row in 0..cols {
        for col in 0..rows {
            if in_bounds(&pol_map, &Point(row as i32, col as i32))
                && get_score(&pol_map, &Point(row as i32, col as i32)) == -1
            {
                flood(&mut pol_map, &Point(row as i32, col as i32));
            }
        }
    }
    draw_map(&pol_map);

    println!(
        "Part 2: {:?}",
        pol_map.iter().flatten().filter(|x| **x == -1).count()
    );
}
