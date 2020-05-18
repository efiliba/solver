extern crate solver;

// use solver::utils::{bit_utils, combinations::Combinations};
use solver::cell::cell::Cell2;
use solver::cell::{cell::Cell, dimensions::Dimensions, SetMethod};
use solver::sub_grid::sub_grid::SubGrid;
use solver::grid::grid::Grid;
use solver::utils::combinations::Combinations;
use solver::utils::array_utils::{square_rows, combine_rows, transpose_rows};


// use solver::utils::bit_utils::highest_bit_position;



  fn it_solves_a_3x1_grid() {
    let columns = 3;
    let rows = 1;
    let dimensions = Dimensions::new(columns, rows);
    let mut grid = Grid::new(&dimensions);
    // assert_eq!(grid.solve(), false);

        println!("{}", &grid);

    grid.set_by_option(0, 0, 0, 0, 1, SetMethod::Loaded); 				  // Set top left cell to 1
    grid.set_by_option(1, 0, 0, 1, 2, SetMethod::Loaded); 				  // Middle cell to 2

        println!("Solved:\n{}", &grid);
    // assert!(grid.solve());
    assert!(false);
  }


fn main() {
  
    let columns = 2;
    let rows = 2;
    let dimensions = Dimensions::new(columns, rows);
    let mut grid = Grid::new(&dimensions);

    grid.fix_by_position(0, 0, 0, 0, 0, 0);                         // Set top left cell to 1
    grid.fix_by_position(0, 0, 1, 0, 1, 0);                         // Set top 2nd cell to 2
    grid.fix_by_position(1, 0, 0, 0, 0, 1);                         // Set top 3rd cell to 3

    print!("{:#}", grid);
  println!("======================================");

}
