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

    grid.set_by_option(0, 0, 0, 0, 1, SetMethod::Loaded); 				  // Set top left cell to 1
    grid.set_by_option(0, 1, 0, 0, 2, SetMethod::Loaded);           // Set next top cell to 2
    grid.set_by_option(0, 2, 0, 0, 4, SetMethod::Loaded);           // Set 3rd cell to 3
    
  // 1  14  14  14  
  // 2  13  13  13  
  // 4  11  11  11  
  // 8   7   7   7  
// grid.ttt();
    println!("{}", &grid);
    grid.solve();
    println!("Solved:\n{}", &grid);


assert!(false);

    //  1 | 2 | 4 | 8 | [ 15 | 15 | 15 ]             1       | [ 14 | 14 | 14 ]
    //  --------------|-----------------       --------------|-----------------
    //  1 | 2 | 4 | 8 | [ 15 | 15 | 15 ]             2       | [ 13 | 13 | 13 ]
    //  --------------|-----------------  ->   --------------|-----------------
    //  1 | 2 | 4 | 8 | [ 15 | 15 | 15 ]             4       | [ 11 | 11 | 11 ]
    //  --------------|-----------------       --------------|-----------------
    //  1 | 2 | 4 | 8 | [ 15 | 15 | 15 ]         |   |   | 8 | [  7 |  7 |  7 ]

    
    // assert!(grid.solve(), "1x4 grid should be solved");
  }

}