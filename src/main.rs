use std::fs;

fn get_rotations(file_name: &str) -> Vec<String> {
    let contents =
        fs::read_to_string(file_name).expect(&format!("Could not find file {}", file_name));

    contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

fn main() {
    println!("Hello, world!");
}
