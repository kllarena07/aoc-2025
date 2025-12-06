use std::cmp::max;

pub struct Part2 {
    pub rotated_worksheet: Vec<Vec<u32>>,
    pub operations: Vec<char>,
}

impl Part2 {
    pub fn default(worksheet: Vec<String>) -> Self {
        let arithmetic_data: Vec<Vec<char>> = worksheet
            .iter()
            .map(|line| line.chars().collect())
            .collect();
        let ad_len = arithmetic_data.len();

        let digits_data: Vec<Vec<char>> = arithmetic_data[..ad_len - 1].to_owned();

        // get the largest length out of all of the digits data
        let largest_length = digits_data
            .iter()
            .fold(digits_data[0].len(), |acc, line| max(acc, line.len()));

        // add padding
        let padded_digits_data: Vec<Vec<char>> = digits_data
            .iter()
            .map(|line| {
                if line.len() < largest_length {
                    let mut padded_line = line.clone();

                    padded_line.resize(largest_length, ' ');

                    padded_line
                } else {
                    line.clone()
                }
            })
            .collect();

        padded_digits_data.iter().for_each(|l| println!("{:?}", l));

        let pd_len = padded_digits_data.len();
        let pd_line_len = padded_digits_data[0].len();

        let mut rotated_worksheet_strs: Vec<String> = vec![];

        for col in 0..pd_line_len {
            let mut column_data: Vec<char> = vec![];
            for row in 0..pd_len {
                column_data.push(padded_digits_data[row][col]);
            }
            let column_digit = column_data.iter().collect::<String>();
            rotated_worksheet_strs.push(column_digit);
        }

        let mut rotated_worksheet: Vec<Vec<u32>> = vec![];

        let mut column: Vec<u32> = vec![];
        for entry in rotated_worksheet_strs {
            let trimmed = entry.trim();

            if trimmed.is_empty() {
                rotated_worksheet.push(column.clone());
                column.clear();
                continue;
            }

            let parsed_digit = trimmed.parse::<u32>().unwrap();
            println!("{}", parsed_digit);

            column.push(parsed_digit);
        }

        if !column.is_empty() {
            rotated_worksheet.push(column);
        }

        let operations: Vec<char> = arithmetic_data[ad_len - 1]
            .iter()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_owned())
            .collect();

        Self {
            rotated_worksheet,
            operations,
        }
    }

    pub fn solve(&self) -> u128 {
        // we can assume that the number of operations = len(rotated_worksheet)
        let mut answer = 0;
        for (idx, entry) in self.rotated_worksheet.iter().enumerate() {
            let operation = &self.operations[idx];

            let mut start = 0 + ((operation == &'*') as u128);

            for digit in entry {
                if operation == &'*' {
                    start *= digit.clone() as u128;
                }
                if operation == &'+' {
                    start += digit.clone() as u128;
                }
            }

            answer += start;
        }

        answer
    }
}
