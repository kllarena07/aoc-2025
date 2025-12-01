use std::fs::{self};

mod part1;
mod part2;

use crate::{part1::Part1, part2::Part2};

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

    let part1 = Part1::default(rotations.clone());
    let part1_solution = part1.solve();

    println!("PART 1: password is {}", part1_solution);

    let part2 = Part2::default(rotations.clone());
    let part2_solution = part2.solve();

    println!("PART 2: password is {}", part2_solution);
}
