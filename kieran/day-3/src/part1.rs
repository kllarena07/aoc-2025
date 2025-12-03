pub struct Part1 {
    pub batteries: Vec<String>,
}

impl Part1 {
    pub fn default(batteries: Vec<String>) -> Self {
        Self { batteries }
    }

    pub fn solve(&self) -> u32 {
        let mut total_joltage: u32 = 0;

        for battery in &self.batteries {
            let battery_len = battery.len();
            let first_slice = &battery;
            let first_largest = self.get_largest_char(first_slice);

            let second_slice = if first_largest.1 < battery_len - 1 {
                &battery[first_largest.1 + 1..] // return right side of slice
            } else {
                &battery[..first_largest.1] // return left side of slice
            };

            let mut second_largest = self.get_largest_char(second_slice);

            if first_largest.1 < battery_len - 1 {
                second_largest.1 += first_largest.1 + 1;
            }

            // order digits by index
            let joltage = if first_largest.1 < second_largest.1 {
                format!("{}{}", first_largest.0, second_largest.0)
                    .parse::<u32>()
                    .expect("Error parsing joltage.")
            } else {
                format!("{}{}", second_largest.0, first_largest.0)
                    .parse::<u32>()
                    .expect("Error parsing joltage.")
            };

            total_joltage += joltage;
        }

        total_joltage
    }

    fn get_largest_char(&self, slice: &str) -> (char, usize) {
        let mut chars = slice.chars();

        let max_char = chars
            .clone()
            .max()
            .expect("Something went wrong obtaining the largest character.");

        let index = chars.position(|p| p == max_char).unwrap();

        (max_char, index)
    }
}
