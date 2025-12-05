use num_bigint::BigUint;
use std::cmp::{max, min};

pub struct Part2 {
    pub ranges: Vec<String>,
}

impl Part2 {
    pub fn default(ranges: Vec<String>) -> Self {
        Self { ranges }
    }

    pub fn solve(&mut self) -> BigUint {
        let mut ranges_stack: Vec<(BigUint, BigUint)> = vec![];

        for range in &self.ranges {
            let sp_range: Vec<BigUint> = range
                .split("-")
                .map(|s| BigUint::parse_bytes(s.as_bytes(), 10).expect("Error parsing to BigUint"))
                .collect();
            let lower = sp_range[0].clone();
            let upper = sp_range[1].clone();

            ranges_stack.push((lower, upper));
        }

        ranges_stack.sort_by(|a, b| a.1.cmp(&b.1));

        let mut sorted = ranges_stack.clone();
        let mut sorted_ranges_stack: Vec<(BigUint, BigUint)> = vec![];

        while !sorted.is_empty() {
            let range = sorted.remove(0);
            let mut lower = range.0.clone();
            let mut upper = range.1.clone();

            let should_merge = if let Some(top) = sorted_ranges_stack.last() {
                let prev_lower = &top.0;
                let prev_upper = &top.1;

                // check if the current range OR the previous range overlap
                // they overlap if they fulfill the following conditions:
                // 1. the CURRENT LOWER BOUND range falls within the previous range
                // 2. the PREVIOUS LOWER BOUND range falls within the current range
                if (prev_lower <= &lower && &lower <= prev_upper)
                    || (&lower <= prev_lower && prev_lower <= &upper)
                {
                    // swap the values of the ranges
                    lower = min(lower, prev_lower.clone());
                    upper = max(upper, prev_upper.clone());
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if should_merge {
                sorted_ranges_stack.pop();
            }

            sorted_ranges_stack.push((lower, upper));
        }

        let mut total = BigUint::from(0u32);

        for pair in sorted_ranges_stack {
            // include +1 here since it excludes the pair.0 from the difference
            let difference = &pair.1 - &pair.0 + BigUint::from(1u32);
            total += difference;
        }

        total
    }
}
