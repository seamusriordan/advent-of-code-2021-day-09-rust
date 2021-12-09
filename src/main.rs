use std::fs;
use day_09_rust::Grid;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let grid = Grid::new(input.lines());

    println!("{}", grid.get_risk_level());
}
