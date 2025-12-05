use std::fs;

use crate::part1::Part1;

mod part1;

fn get_ingredients(file_name: &str) -> Vec<String> {
    let expect_msg = format!("Error: Could not find file {}.", file_name);
    let contents = fs::read_to_string(file_name).expect(&expect_msg);

    contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

fn get_ranges(file_name: &str) -> Vec<String> {
    let expect_msg = format!("Error: Could not find file {}.", file_name);
    let contents = fs::read_to_string(file_name).expect(&expect_msg);

    contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

fn main() {
    let ranges = get_ranges("./input/ranges.txt");
    let ingredients = get_ingredients("./input/ingredients.txt");

    let part1 = Part1::default(ranges.clone(), ingredients.clone());
    let part1_solution = part1.solve();

    println!("PART 1 Answer: {}", part1_solution);
}
