use std::fs::{self};

fn get_rotations(file_name: &str) -> Vec<String> {
    let expect_msg = format!("Error: Could not find file {}.", file_name);
    let contents = fs::read_to_string(file_name).expect(&expect_msg);

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

fn parse_rotation(rotation: &str) -> (char, i32) {
    let dir_expect_msg = "Error: could not parse direction.";
    let clicks_expect_msg = "Error: could not parse clicks.";

    let mut rotation_chars = rotation.chars();
    let direction = rotation_chars.nth(0).expect(dir_expect_msg);
    let clicks: i32 = rotation[1..]
        .to_owned()
        .parse::<i32>()
        .expect(clicks_expect_msg);

    (direction, clicks)
}

fn main() {
    let mut dial: i32 = 50;
    let mut password: u32 = 0;
    let rotations = get_rotations("./input.txt");

    for rotation in rotations {
        let (direction, clicks) = parse_rotation(&rotation);

        turn_dial(&mut password, &mut dial, &direction, clicks);
    }

    println!("Password is {}", password);
}
