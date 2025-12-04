pub struct Part1 {
    pub paper_rolls: Vec<Vec<char>>,
    pub verbose: bool,
}

#[allow(dead_code)]
impl Part1 {
    pub fn default(paper_rolls: Vec<Vec<char>>) -> Self {
        Self {
            paper_rolls,
            verbose: false,
        }
    }

    pub fn configured(paper_rolls: Vec<Vec<char>>, verbose: bool) -> Self {
        Self {
            paper_rolls,
            verbose,
        }
    }

    pub fn solve(&self) -> u32 {
        let paper_rolls_len = self.paper_rolls.len();
        let mut num_of_accessible_rolls = 0;

        for curr_y in 0..paper_rolls_len {
            let line_len = self.paper_rolls[curr_y].len();

            if self.verbose {
                print!("{:?}", self.paper_rolls[curr_y]);
            }
            for curr_x in 0..line_len {
                if self.paper_rolls[curr_y][curr_x] != '@' {
                    continue;
                }

                let total_adjacent_rolls =
                    self.check_adjacent(curr_y, curr_x, line_len, paper_rolls_len);

                if total_adjacent_rolls < 4 {
                    if self.verbose {
                        // print!(" ({}, {})", curr_x, paper_rolls[curr_y][curr_x]);
                    }
                    num_of_accessible_rolls += 1;
                }
            }
            if self.verbose {
                println!("");
            }
        }

        num_of_accessible_rolls
    }

    fn check_left(&self, curr_y: usize, curr_x: usize) -> bool {
        if curr_x > 0 {
            let left = curr_x - 1;
            self.paper_rolls[curr_y][left] == '@'
        } else {
            false
        }
    }

    fn check_right(&self, curr_y: usize, curr_x: usize, line_len: usize) -> bool {
        if curr_x < line_len - 1 {
            let right = curr_x + 1;
            self.paper_rolls[curr_y][right] == '@'
        } else {
            false
        }
    }

    fn check_up(&self, curr_y: usize, curr_x: usize) -> bool {
        if curr_y > 0 {
            let up = curr_y - 1;
            self.paper_rolls[up][curr_x] == '@'
        } else {
            false
        }
    }

    fn check_up_left(&self, curr_y: usize, curr_x: usize) -> bool {
        if curr_y > 0 && curr_x > 0 {
            let up = curr_y - 1;
            let left = curr_x - 1;
            self.paper_rolls[up][left] == '@'
        } else {
            false
        }
    }

    fn check_up_right(&self, curr_y: usize, curr_x: usize, line_len: usize) -> bool {
        if curr_y > 0 && curr_x < line_len - 1 {
            let up = curr_y - 1;
            let right = curr_x + 1;
            self.paper_rolls[up][right] == '@'
        } else {
            false
        }
    }

    fn check_down(&self, curr_y: usize, curr_x: usize, paper_rolls_len: usize) -> bool {
        if curr_y < paper_rolls_len - 1 {
            let down = curr_y + 1;
            self.paper_rolls[down][curr_x] == '@'
        } else {
            false
        }
    }

    fn check_down_left(&self, curr_y: usize, curr_x: usize, paper_rolls_len: usize) -> bool {
        if curr_y < paper_rolls_len - 1 && curr_x > 0 {
            let down = curr_y + 1;
            let left = curr_x - 1;
            self.paper_rolls[down][left] == '@'
        } else {
            false
        }
    }

    fn check_down_right(
        &self,
        curr_y: usize,
        curr_x: usize,
        line_len: usize,
        paper_rolls_len: usize,
    ) -> bool {
        if curr_y < paper_rolls_len - 1 && curr_x < line_len - 1 {
            let down = curr_y + 1;
            let right = curr_x + 1;
            self.paper_rolls[down][right] == '@'
        } else {
            false
        }
    }

    fn check_adjacent(
        &self,
        curr_y: usize,
        curr_x: usize,
        line_len: usize,
        paper_rolls_len: usize,
    ) -> u32 {
        let mut total_adjacent_rolls = 0;

        if self.check_left(curr_y, curr_x) {
            total_adjacent_rolls += 1;
        }

        if self.check_right(curr_y, curr_x, line_len) {
            total_adjacent_rolls += 1;
        }

        if self.check_up(curr_y, curr_x) {
            total_adjacent_rolls += 1;
        }

        if self.check_up_left(curr_y, curr_x) {
            total_adjacent_rolls += 1;
        }

        if self.check_up_right(curr_y, curr_x, line_len) {
            total_adjacent_rolls += 1;
        }

        if self.check_down(curr_y, curr_x, paper_rolls_len) {
            total_adjacent_rolls += 1;
        }

        if self.check_down_left(curr_y, curr_x, paper_rolls_len) {
            total_adjacent_rolls += 1;
        }

        if self.check_down_right(curr_y, curr_x, line_len, paper_rolls_len) {
            total_adjacent_rolls += 1;
        }

        total_adjacent_rolls
    }
}
