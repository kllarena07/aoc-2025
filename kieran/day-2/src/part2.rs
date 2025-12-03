pub struct Part2 {
    pub ranges: Vec<String>,
}

impl Part2 {
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
        // - odd OR even string length
        // - symmetric
        // it is made only of some sequence of digits repeated at least twice

        let mut invalid_ids: Vec<u64> = vec![];

        for id in lower..=upper {
            let id_as_string = id.to_string();
            let id_slen = id_as_string.len();
            let max_digit_combo: usize = id_slen / 2;

            'outer: for i in 1..=max_digit_combo {
                let repeated_subsequence = &id_as_string[..i];
                let rs_len = repeated_subsequence.len();

                if id_slen % rs_len > 0 {
                    // go to the next repeated subsequence combination
                    continue;
                }

                let mut j = i;

                while j < id_slen {
                    let cmp = &id_as_string[j..j + rs_len];
                    if repeated_subsequence != cmp {
                        // go to the next repeated subsequence combination
                        continue 'outer;
                    }

                    j += rs_len;
                }

                invalid_ids.push(id);
                break;
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
