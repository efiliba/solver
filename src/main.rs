extern crate solver;

// use solver::utils::{bit_utils, combinations::Combinations};
use solver::cell::cell::Cell2;
use solver::cell::{cell::Cell, dimensions::Dimensions};
use solver::sub_grid::sub_grid::SubGrid;
use solver::grid::grid::Grid;
use solver::utils::combinations::Combinations;

// use solver::utils::bit_utils::highest_bit_position;

	fn select_elements<'a, T>(from: &'a Vec<T>, select: usize) -> Vec<&'a T> {
		let mut elements = Vec::with_capacity(from.len());
    for index in 0..from.len() {
			if (1 << index) & select > 0 {
				elements.push(&from[index]);
			}
		}

		elements
	}

fn main() {
  
  // assert_eq!(&format!("{:#}", Foo(23)), "Foo(23)");
  // assert_eq!(&format!("{}", Foo(23)), "23");
  // println!("1: {:#}", Foo {bar: 23});
  // println!("2: {}", Foo {bar: 24});
  // let combinations = Combinations::new(4);                          // Columns * rows

  // println!("Bits set in main: {}", bit_utils::number_of_bits_set(333)); // 333 = 101001101

  println!("======================================");

    let combinations = Combinations::new(4);
    let from = vec!['1', '2', '3'];
		let pick = 2;
    let actual = combinations.select(&from, pick);
  
    
    println!("{:?}", actual);


  // let mut cell2 = Cell2::new(1, 1);
  // match cell2 {
  //   Cell2::OptionsCell { column, row } => println!("OptionsCell: ({}:{})", column, row),
  //   Cell2::SetCell { symbol } => println!("Symbol: {}", symbol),
  // };

  // cell2.change();
  // match cell2 {
  //   Cell2::OptionsCell { column, row } => println!("OptionsCell: ({}:{})", column, row),
  //   Cell2::SetCell { symbol } => println!("Symbol: {}", symbol),
  // };

  // println!("======================================\n");

  
  println!("__________________");

  // cell.json.print();
}
