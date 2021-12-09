use std::str::Lines;

mod tests;

pub struct Grid {
    values: Vec<Vec<i32>>,
}

impl Grid {
    pub fn new(lines: Lines) -> Grid {
        let mut grid = Grid {
            values: Vec::<Vec<i32>>::new()
        };

        for line in lines {
            let mut row = Vec::<i32>::new();
            for c in line.chars() {
                row.push(String::from(c).parse::<i32>().unwrap())
            }
            grid.values.push(row);
        }

        grid
    }

    pub fn get_value(&self, i: i32, j: i32) -> Option<i32> {
        if j < 0 || j >= self.values.len() as i32 {
            return None;
        }

        if i < 0 || i >= self.values[j as usize].len() as i32 {
            return None;
        }

        Some(self.values[j as usize][i as usize])
    }

    pub fn get_risk_level(&self) -> i32 {
        let mut risk = 0;

        for j in 0..self.values.len() {
            for i in 0..self.values[j].len() {
                risk += self.get_risk_for_point(i, j);
            }
        }

        risk
    }

    fn get_risk_for_point(&self, i: usize, j: usize) -> i32 {
        let i_i32 = i as i32;
        let j_i32 = j as i32;

        let adjacent_points = vec![
            (i_i32 - 1, j_i32),
            (i_i32, j_i32 - 1),
            (i_i32 + 1, j_i32),
            (i_i32, j_i32 + 1),
        ];

        let center_value = self.get_value(i_i32, j_i32).unwrap();

        for p in adjacent_points {
            if let Some(adjacent_value) = self.get_value(p.0, p.1) {
                if adjacent_value <= center_value {
                    return 0;
                }
            }
        }
        center_value + 1
    }
}
