use std::fs::{self};

use crate::part1::Part1;

mod part1;

fn get_rotations(file_name: &str) -> Vec<String> {
    let expect_msg = format!("Error: Could not find file {}.", file_name);
    let contents = fs::read_to_string(file_name).expect(&expect_msg);

    contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

fn main() {
    let rotations = get_rotations("./input.txt");

    let part1 = Part1::default(rotations);
    let part1_solution = part1.solve();

    println!("PART 1: password is {}", part1_solution);
}
