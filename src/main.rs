extern crate solver;

use solver::utils::{bit_utils, combinations::Combinations};
use solver::cell;

fn main() {
  let combinations = Combinations::new(4);                          // Columns * rows

  println!("Bits set in main: {}", bit_utils::number_of_bits_set(333)); // 333 = 101001101
  cell::test();

  let from = ['a', 'b', 'c', 'd'].to_vec();
  let pick = 4;
  let items = combinations.select(from, pick);

  // const expected: string[][] = [];
  // expected[0] = ["a"];
  // expected[1] = ["b"];
  // expected[2] = ["c"];
  // expected[3] = ["d"];

  print!("=============={:?}", items);

  // combinations.test();
}
