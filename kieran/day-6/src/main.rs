use std::fs;

mod part1;
mod part2;

use crate::part1::Part1;
use crate::part2::Part2;

fn get_worksheet(file_name: &str) -> Vec<String> {
    let expect_msg = format!("Error: Could not find file {}.", file_name);
    let contents = fs::read_to_string(file_name).expect(&expect_msg);

    contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

fn main() {
    let worksheet = get_worksheet("./input.txt");

    let part1 = Part1::default(worksheet.clone());
    let part1_solution = part1.solve();

    println!("PART 1 Answer: {part1_solution}");

    let part2 = Part2::default(worksheet.clone());
    let part2_solution = part2.solve();

    println!("PART 2 Answer: {part2_solution}");
}
