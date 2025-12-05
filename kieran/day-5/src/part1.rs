use std::collections::HashSet;

pub struct Part1 {
    pub ranges: Vec<String>,
    pub ingredients: Vec<String>,
}

impl Part1 {
    pub fn default(ranges: Vec<String>, ingredients: Vec<String>) -> Self {
        Self {
            ranges,
            ingredients,
        }
    }

    pub fn solve(&self) -> usize {
        let mut fresh_set: HashSet<u128> = HashSet::new();

        for range in &self.ranges {
            let s_range: Vec<String> = range.split("-").map(|s| s.to_owned()).collect();
            let lower = s_range[0].parse::<u128>().unwrap();
            let upper = s_range[1].parse::<u128>().unwrap();

            for ingredient in &self.ingredients {
                let p_ingredient = ingredient.parse::<u128>().unwrap();

                if lower <= p_ingredient && p_ingredient <= upper {
                    fresh_set.insert(p_ingredient);
                }
            }
        }

        fresh_set.len()
    }
}
