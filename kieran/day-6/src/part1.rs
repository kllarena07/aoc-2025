pub struct Part1 {
    pub arithmetic_data: Vec<Vec<String>>,
}

impl Part1 {
    pub fn default(worksheet: Vec<String>) -> Self {
        let arithmetic_data: Vec<Vec<String>> = worksheet
            .iter()
            .map(|line| {
                line.split(" ")
                    .filter(|char| !char.is_empty())
                    .map(|char| char.to_owned())
                    .collect()
            })
            .collect();

        Self { arithmetic_data }
    }

    pub fn solve(&self) -> u128 {
        let max_cols = self
            .arithmetic_data
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap();
        let columns_data: Vec<Vec<String>> = (0..max_cols)
            .map(|col_idx| {
                self.arithmetic_data
                    .iter()
                    .rev()
                    .filter_map(|row| row.get(col_idx).cloned())
                    .collect()
            })
            .collect();

        let answer = columns_data.iter().fold(0, |answer, line| {
            let operation = line[0].as_str();
            // starting is 1 if operation is multiplication
            // going of the assumption that operation can ONLY be either * or +
            let start = 0 + ((operation == "*") as u128);

            let accumulator = line[1..].iter().fold(start, |acc, s_num| {
                let p_num = s_num.parse::<u128>().unwrap();

                if operation == "*" {
                    acc * p_num
                } else {
                    acc + p_num
                }
            });

            answer + accumulator
        });

        answer
    }
}
