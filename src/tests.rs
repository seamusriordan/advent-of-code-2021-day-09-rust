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

        assert_eq!(2, grid.get_value(0,0).unwrap());
        assert_eq!(8, grid.get_value(9,4).unwrap());
    }

    #[test]
    fn example_has_four_low_points() {
        let lines =
            "2199943210
3987894921
9856789892
8767896789
9899965678".lines();

        let grid = Grid::new(lines);

        let basins = grid.get_basins();
        assert_eq!(4, basins.len());
    }

    #[test]
    fn top_three_basins_are_14_9_9() {
        let lines =
            "2199943210
3987894921
9856789892
8767896789
9899965678".lines();

        let grid = Grid::new(lines);

        let basins = grid.get_basins();

        assert_eq!(14, basins[0].len());
        assert_eq!(9, basins[1].len());
        assert_eq!(9, basins[2].len());
    }
}