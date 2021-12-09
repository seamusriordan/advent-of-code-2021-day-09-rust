#[cfg(test)]

mod tests {
    use crate::Grid;

    #[test]
    fn grid_constructor() {
        let lines =
        "2199943210
3987894921
9856789892
8767896789
9899965678".lines();

        let grid = Grid::new(lines);

        assert_eq!(2, grid.get_value(0,0));
        assert_eq!(8, grid.get_value(9,4));
    }
}