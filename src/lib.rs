use std::str::Lines;

mod tests;

struct Grid {
    values: Vec<Vec<i32>>
}

impl Grid {
    fn new(lines: Lines) -> Grid {
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

    fn set_values(&mut self, values: &Vec<Vec<i32>>) {
        for (i, new_value) in values.clone().into_iter().enumerate() {
            self.values[i] = new_value.clone()
        }
    }

    fn get_value(&self, i: usize, j: usize) -> i32 {
        self.values[j][i]
    }
}

fn find_low_points() {

}