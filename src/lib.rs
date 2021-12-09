use std::str::Lines;

mod tests;

pub struct Grid {
    values: Vec<Vec<i32>>
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

    pub fn get_value(&self, i: usize, j: usize) -> i32 {
        self.values[j][i]
    }
}

fn find_low_points() {

}