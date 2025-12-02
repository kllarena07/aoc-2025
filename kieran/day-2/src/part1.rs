pub struct Part1 {
    pub ranges: Vec<String>,
}

impl Part1 {
    pub fn default(ranges: Vec<String>) -> Self {
        Self { ranges }
    }

    pub fn solve(&self) -> u64 {
        let mut invalid_ids: Vec<u64> = vec![];
        let mut total: u64 = 0;

        for id_range in &self.ranges {
            let range_invalid_ids: Vec<u64> = self.get_invalid_ids(&id_range);
            invalid_ids.extend_from_slice(&range_invalid_ids);
        }

        for val in invalid_ids {
            total += val;
        }

        total
    }

    fn get_invalid_ids(&self, range: &str) -> Vec<u64> {
        let (lower, upper) = self.parse_range(range);

        // invalid ids are:
        // - only EVEN
        // - symmetric

        let mut invalid_ids: Vec<u64> = vec![];

        for id in lower..=upper {
            let id_as_string = id.to_string();
            let id_slen = id_as_string.len();
            let half_len = id_slen / 2;
            let first_half = &id_as_string[0..half_len];
            let second_half = &id_as_string[half_len..];

            if id_slen % 2 == 0 {
                if first_half == second_half {
                    invalid_ids.push(id);
                }
            }
        }

        invalid_ids
    }

    fn parse_range(&self, range: &str) -> (u64, u64) {
        let parse_expect_msg = format!("Error parsing range: {range}");

        range
            .split_once("-")
            .map(|(lower, upper)| {
                let lower_expect_msg = format!("Error parsing lower bound for range: {range}");
                let upper_expect_msg = format!("Error parsing upper bound for range: {range}");
                (
                    lower.parse::<u64>().expect(&lower_expect_msg),
                    upper.parse::<u64>().expect(&upper_expect_msg),
                )
            })
            .expect(&parse_expect_msg)
    }
}
