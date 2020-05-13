#[cfg(test)]
use crate::cell::dimensions::Dimensions;
use crate::sub_grid::sub_grid::SubGrid;

#[cfg(test)]
fn init_sub_grids(dimensions: &Dimensions) -> Vec<Vec<SubGrid>> {
  let mut sub_grids: Vec<Vec<SubGrid>> = Vec::with_capacity(dimensions.total);
  let swopped = dimensions.swop();
  for row in 0..dimensions.rows {
    sub_grids.push(Vec::with_capacity(dimensions.columns));
    for column in 0..dimensions.columns {
      sub_grids[row].push(SubGrid::new(swopped, row, column));      // Columns and rows transposed
    }
  }

  sub_grids
}

#[cfg(test)]
mod grid {
  use crate::cell::{dimensions::Dimensions, SetMethod};
  use crate::grid::grid::Grid;

  #[test]
  fn it_creates_a_4x1_grid() {
    let columns = 4;
    let rows = 1;
    let dimensions = Dimensions::new(columns, rows);
    let grid = Grid::new(&dimensions);

    let expected_sub_grids = super::init_sub_grids(&dimensions);
    assert!(grid.compare(&expected_sub_grids));
  }

  #[test]
  fn it_solves_a_1x4_grid() {
    let columns = 1;
    let rows = 4;
    let dimensions = Dimensions::new(columns, rows);
    let mut grid = Grid::new(&dimensions);

    // let expected_sub_grids = super::init_sub_grids(&dimensions);
    // assert!(grid.compare(&expected_sub_grids));

    	// 	// it("should be solved", () => {
    grid.set_by_option(0, 0, 0, 0, 1, SetMethod::Loaded); 				// Set top left cell to 1
    // grid.set_by_option(0, 1, 0, 0, 2, SetMethod::Loaded);            // Set next top cell to 2
    // grid.set_by_option(0, 2, 0, 0, 4, SetMethod::Loaded);            // Set 3rd cell to 3
    // grid.solve();

			// //  1 | 2 | 4 | 8 | [ 15 | 15 | 15 ]             1       | [ 14 | 14 | 14 ]
			// //  --------------|-----------------       --------------|-----------------
			// //  1 | 2 | 4 | 8 | [ 15 | 15 | 15 ]             2       | [ 13 | 13 | 13 ]
			// //  --------------|-----------------  ->   --------------|-----------------
			// //  1 | 2 | 4 | 8 | [ 15 | 15 | 15 ]             4       | [ 11 | 11 | 11 ]
			// //  --------------|-----------------       --------------|-----------------
			// //  1 | 2 | 4 | 8 | [ 15 | 15 | 15 ]         |   |   | 8 | [  7 |  7 |  7 ]

			// expectedSubGrids[0][0].setByOption(0, 0, 1, SetMethod.user);	// Top left cells set to 1, 2, 4 and 8
			// expectedSubGrids[1][0].setByOption(0, 0, 2, SetMethod.user);
			// expectedSubGrids[2][0].setByOption(0, 0, 4, SetMethod.user);
			// expectedSubGrids[3][0].setByOption(0, 0, 8, SetMethod.user);
			// expectedSubGrids[0][0].simplify();                            // Sets other cells to 14, 13, 11 and 7
			// expectedSubGrids[1][0].simplify();
			// expectedSubGrids[2][0].simplify();
			// expectedSubGrids[3][0].simplify();                                                      
			// expect(grid.compare(expectedSubGrids)).toBeTruthy();
		// });
  }

}