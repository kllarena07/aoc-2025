pub struct Part2 {
    pub batteries: Vec<String>,
}

impl Part2 {
    pub fn default(batteries: Vec<String>) -> Self {
        Self { batteries }
    }

    pub fn solve(&self) -> u128 {
        let mut total_joltage: u128 = 0;

        // unfortunately could not solve this one
        // solution logic comes from:
        // - https://github.com/jakewaldrip/aoc-2025/blob/main/src/days/day03.rs#L87
        for battery in &self.batteries {
            let mut removals_remaining = battery.len() - 12;
            let mut stack: Vec<i32> = Vec::new();

            for char in battery.chars() {
                let parsed_char = String::from(char).parse::<i32>().unwrap();
                while let Some(&top) = stack.last() {
                    if removals_remaining == 0 || parsed_char <= top {
                        break;
                    }

                    stack.pop();
                    removals_remaining -= 1;
                }

                stack.push(parsed_char);
            }

            let final_battery: String = stack
                .iter()
                .map(|num| num.to_string())
                .collect::<String>()
                .parse()
                .unwrap();

            let joltage: u128 = String::from(&final_battery[..12]).parse::<u128>().unwrap();

            total_joltage += joltage;
        }

        total_joltage
    }
}
