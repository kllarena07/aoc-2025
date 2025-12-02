use std::fs::{self};

mod part1;

use crate::part1::Part1;

fn get_id_ranges(file_name: &str) -> Vec<String> {
    let expect_msg = format!("Error: Could not find file {}.", file_name);
    let contents = fs::read_to_string(file_name).expect(&expect_msg);

    contents
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_owned())
        .collect()
}

fn main() {
    let id_ranges = get_id_ranges("./input.txt");

    let part1 = Part1::default(id_ranges);
    let total = part1.solve();

    println!("PART 1 Answer: {total}");

    // invalid ids
    // any ID which is made only of some sequence of digits repeated twice.
    // So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
    //
    // None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

    // return the sum of all the invalid IDs
}
