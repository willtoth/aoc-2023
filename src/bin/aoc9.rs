use std::fs;

fn delta_array(arr: &Vec<i32>) -> Vec<i32> {
    arr.iter()
        .skip(1)
        .scan(arr[0], |state, x| {
            let result = x - *state;
            *state = *x;
            Some(result)
        })
        .collect::<Vec<i32>>()
}

fn next_value(arr: &Vec<i32>) -> i32 {
    if arr.iter().all(|x| *x == 0) {
        return 0;
    }
    let next = next_value(&delta_array(&arr));
    arr.last().unwrap() + next
}

fn prev_value(arr: &Vec<i32>) -> i32 {
    if arr.iter().all(|x| *x == 0) {
        return 0;
    }
    let next = prev_value(&delta_array(&arr));
    arr.first().unwrap() - next
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input = input
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let part1 = input.iter().map(|x| next_value(x)).sum::<i32>();
    let part2 = input.iter().map(|x| prev_value(x)).sum::<i32>();

    println!("{:?}", part1);
    println!("{:?}", part2);
}
