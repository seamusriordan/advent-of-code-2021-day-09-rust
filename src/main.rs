use std::fs;
use day_09_rust::Grid;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let grid = Grid::new(input.lines());

    let basins = grid.get_basins();
    println!("{}", basins[0].len()*basins[1].len()*basins[2].len());
}
