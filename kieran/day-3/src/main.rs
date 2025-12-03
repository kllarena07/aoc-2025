use std::fs;

mod part1;
use crate::part1::Part1;

fn get_batteries(file_name: &str) -> Vec<String> {
    let expect_msg = format!("Error: Could not find file {}.", file_name);
    let contents = fs::read_to_string(file_name).expect(&expect_msg);

    contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

fn main() {
    let batteries: Vec<String> = get_batteries("./input.txt");

    let part1 = Part1::default(batteries);
    let part1_solution = part1.solve();

    println!("PART 1 Answer: {part1_solution}");
}
