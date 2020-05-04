extern crate solver;

use solver::utils::{bit_utils, combinations::Combinations};
use solver::cell::cell::{Cell, Dimensions};

fn main() {
  // let combinations = Combinations::new(4);                          // Columns * rows

  // println!("Bits set in main: {}", bit_utils::number_of_bits_set(333)); // 333 = 101001101
  // cell::test();

  let dimensions = &Dimensions::new(2, 2);

  let mut cell = Cell::new(dimensions, 1, 1);
  
  // cell.reset();
  cell.test();

  println!("Column: {}", cell.column);
  println!("Row: {}", cell.row);
  println!("Solved: {}", cell.solved());

  let last_option_found = cell.remove_option_at_position(0, 0);
  cell.test();
  println!("Last option: {}", last_option_found);

  let last_option_found = cell.remove_option_at_position(0, 1);
  cell.test();
  println!("Last option: {}", last_option_found);

  let last_option_found = cell.remove_option_at_position(1, 0);
  cell.test();
  println!("Last option: {}", last_option_found);
  println!("Solved: {}", cell.solved());

  println!("__________________");

  cell.json.print();


  // let from = ['a', 'b', 'c', 'd'].to_vec();
  // let pick = 3;
  // let items = combinations.select(from, pick);

  // const expected: string[][] = [];
  // expected[0] = ["a"];
  // expected[1] = ["b"];
  // expected[2] = ["c"];
  // expected[3] = ["d"];

  // print!("=============={:?}", items);

  // combinations.test();
}
