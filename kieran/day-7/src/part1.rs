pub struct Part1 {
    pub diagram: Vec<Vec<char>>,
}

impl Part1 {
    pub fn default(diagram: Vec<Vec<char>>) -> Self {
        Self { diagram }
    }

    pub fn solve(&self) -> usize {
        let starting_point = self.diagram[0]
            .iter()
            .position(|c| c == &'S')
            .expect("Couldn't find the starting point. S may not be in this row or exist.");
        // row length is uniform
        let columns = self.diagram.len();
        let rows = self.diagram[0].len();

        let mut points: Vec<(usize, usize)> = vec![(starting_point, 1)];
        let mut visited: Vec<(usize, usize)> = vec![];

        while !points.is_empty() {
            let front = points[0];
            let x = front.0;
            let y = front.1;

            let mut to_add: Vec<(usize, usize)> = vec![];

            if self.diagram[y][x] != '^' && y < columns - 1 {
                points[0].1 += 1;
            } else {
                let removed = points.remove(0);

                if self.diagram[y][x] == '^' {
                    if !visited.contains(&(x, y)) {
                        visited.push((x, y));
                    }
                    println!("({}, {})", x, y);
                    if removed.0 > 0 {
                        to_add.push((removed.0 - 1, removed.1));
                    }

                    if removed.0 + 1 < rows - 1 {
                        to_add.push((removed.0 + 1, removed.1));
                    }
                }
            }

            for point in to_add {
                if !points.contains(&point) {
                    points.push(point);
                }
            }
        }
        visited.len()
    }
}
