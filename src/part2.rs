pub struct Part2 {
    pub rotations: Vec<String>,
    pub verbose: bool,
}

#[allow(dead_code)]
impl Part2 {
    pub fn default(rotations: Vec<String>) -> Self {
        Self {
            rotations,
            verbose: false,
        }
    }

    pub fn configured(rotations: Vec<String>, verbose: bool) -> Self {
        Self { rotations, verbose }
    }

    pub fn solve(&self) -> u32 {
        let mut dial: i32 = 50;
        let mut password: u32 = 0;

        for rotation in &self.rotations {
            let (direction, clicks) = self.parse_rotation(rotation);

            self.turn_dial(&mut password, &mut dial, &direction, clicks);
        }

        password
    }

    fn turn_dial(&self, password: &mut u32, dial: &mut i32, direction: &char, clicks: i32) {
        // L = -
        // R = +
        // dial cannot go below 0 or above 99
        // if it goes below 0, it goes to 99
        // if it goes above 99, it goes to 0

        for _ in 0..clicks {
            // count the number of times any click causes the dial to point at 0
            if *dial == 0 {
                *password += 1;
            }

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

        if self.verbose {
            println!("The dial is rotated {direction}{clicks} to point at {dial}");
        }
    }

    fn parse_rotation(&self, rotation: &str) -> (char, i32) {
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
}
