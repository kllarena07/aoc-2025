use std::fs;

mod part1;
use crate::part1::Part1;

fn get_paper_rolls(file_name: &str) -> Vec<Vec<char>> {
    let expect_msg = format!("Error: Could not find file {}.", file_name);
    let contents = fs::read_to_string(file_name).expect(&expect_msg);

    let lines: Vec<String> = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect();

    let paper_map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut nested_vec: Vec<char> = vec![];
            let chars = line.chars();

            for char in chars {
                nested_vec.push(char);
            }

            nested_vec
        })
        .collect();

    paper_map
}

fn main() {
    let paper_rolls = get_paper_rolls("./input.txt");

    let part1 = Part1::default(paper_rolls.clone());
    let part1_solution = part1.solve();

    println!("PART 1 Anser: {part1_solution}");
}
