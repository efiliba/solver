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
    let rows = 3;
    let dimensions = Dimensions::new(columns, rows);
    let mut grid = Grid::new(&dimensions);

    grid.set_by_symbol(0, 0, 2, 1, '2', SetMethod::Loaded);
    grid.set_by_symbol(1, 0, 0, 0, '1', SetMethod::Loaded);
    grid.set_by_symbol(1, 0, 2, 1, '3', SetMethod::Loaded);
    grid.set_by_symbol(0, 1, 0, 0, '4', SetMethod::Loaded);
    grid.set_by_symbol(0, 1, 2, 1, '3', SetMethod::Loaded);
    grid.set_by_symbol(1, 1, 0, 0, '3', SetMethod::Loaded);
    grid.set_by_symbol(1, 1, 2, 1, '5', SetMethod::Loaded);
    grid.set_by_symbol(0, 2, 0, 0, '3', SetMethod::Loaded);
    grid.set_by_symbol(0, 2, 2, 1, '1', SetMethod::Loaded);
    grid.set_by_symbol(1, 2, 0, 0, '6', SetMethod::Loaded);

    // [48, 60, 56, 1, 58, 42]
    // [49, 57, 2, 24, 56, 4]
    // [8, 51, 48, 4, 35, 35]
    // [35, 35, 4, 10, 43, 16]
    // [4, 26, 24, 32, 27, 11]
    // [50, 58, 1, 26, 30, 10]

    grid.solve();

    // [48, 4, 56, 1, 58, 42]       <- 60 changed to 4: correct
    // [49, 57, 2, 24, 56, 4]
    // [8, 51, 48, 4, 35, 35]
    // [35, 35, 4, 10, 43, 16]
    // [4, 26, 24, 32, 27, 11]
    // [50, 58, 1, 26, 4, 10]       <- 30 changed to 4: correct

    print!("{:#}", grid);
  println!("======================================");

}
