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

fn turn_dial(password: &mut u32, dial: &mut i32, direction: &char, clicks: i32) {
    // L = -
    // R = +
    // dial cannot go below 0 or above 99
    // if it goes below 0, it goes to 99
    // if it goes above 99, it goes to 0

    for _ in 0..clicks {
        if direction == &'L' {
            *dial -= 1;

            if *dial < 0 {
                *dial = 99;
            }
        } else if direction == &'R' {
            *dial += 1;

            if *dial > 99 {
                *dial = 0;
            }
        }
    }

    // The actual password is the
    // number of times the dial is left pointing
    // at 0 after any rotation in the sequence.
    if *dial == 0 {
        *password += 1;
    }

    println!("The dial is rotated {direction}{clicks} to point at {dial}");
}

fn main() {
    println!("Hello, world!");
}
