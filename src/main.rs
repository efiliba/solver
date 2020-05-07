extern crate solver;

// use solver::utils::{bit_utils, combinations::Combinations};
use solver::cell::cell::Cell2;
use solver::cell::{cell::Cell, dimensions::Dimensions};
use solver::sub_grid::SubGrid;

// use solver::utils::bit_utils::highest_bit_position;

fn main() {
  // let combinations = Combinations::new(4);                          // Columns * rows

  // println!("Bits set in main: {}", bit_utils::number_of_bits_set(333)); // 333 = 101001101
  // cell::test();
  println!("======================================");

  let dimensions = &Dimensions::new(2, 2);

  let subgid = SubGrid::new(dimensions, 0, 0);
  println!("subgid {:#?}", subgid);

  let mut cell2 = Cell2::new(1, 1);
  match cell2 {
    Cell2::OptionsCell { column, row } => println!("OptionsCell: ({}:{})", column, row),
    Cell2::SetCell { symbol } => println!("Symbol: {}", symbol),
  };

  cell2.change();
  match cell2 {
    Cell2::OptionsCell { column, row } => println!("OptionsCell: ({}:{})", column, row),
    Cell2::SetCell { symbol } => println!("Symbol: {}", symbol),
  };

  println!("======================================\n");


  let cell = Cell::new(dimensions, 0, 0);
  println!("options: {}", cell.options);
  // cell.reset();
  // cell.test();

  // let last_option_found = cell.remove_option_at_position(0, 0);
  // cell.test();
  // println!("Last option: {}", last_option_found);

  // let last_option_found = cell.remove_option_at_position(0, 1);
  // cell.test();
  // println!("Last option: {}", last_option_found);

  // let last_option_found = cell.remove_option_at_position(1, 0);
  // cell.test();
  // println!("Last option: {}", last_option_found);
  // println!("Solved: {}", cell.solved());


    // assert_eq!(highest_bit_position(2), 0);
  
  println!("__________________");

  // cell.json.print();
}
