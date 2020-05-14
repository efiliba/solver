extern crate solver;

// use solver::utils::{bit_utils, combinations::Combinations};
use solver::cell::cell::Cell2;
use solver::cell::{cell::Cell, dimensions::Dimensions};
use solver::sub_grid::sub_grid::SubGrid;
use solver::grid::grid::Grid;

// use solver::utils::bit_utils::highest_bit_position;

fn init_sub_grids(dimensions: &Dimensions) -> Vec<Vec<SubGrid>> {
  let mut sub_grids: Vec<Vec<SubGrid>> = Vec::with_capacity(dimensions.total);
  let swopped = dimensions.swop();
  for row in 0..dimensions.rows {
    sub_grids.push(Vec::with_capacity(dimensions.columns));
    for column in 0..dimensions.columns {
      sub_grids[row].push(SubGrid::new(swopped, column, row));
    }
  }

  sub_grids
}


  fn it_creates_a_4x1_grid() {
    let columns = 2;
    let rows = 1;
    let dimensions = Dimensions::new(columns, rows);
    let grid = Grid::new(&dimensions);

    // Ensure a 2 x 4 sub grid created
    let expected_sub_grids = init_sub_grids(&dimensions);
    assert!(grid.compare(&expected_sub_grids));
    // println!("{:#?}", grid);
    // assert!(false);
  }

  use std::fmt;

struct Foo {
  bar: usize
}

impl fmt::Display for Foo {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if formatter.alternate() {
            write!(formatter, "Foo({})", self.bar)
        } else {
            write!(formatter, "{}", self.bar)
        }
    }
}

fn main() {
  
  // assert_eq!(&format!("{:#}", Foo(23)), "Foo(23)");
  // assert_eq!(&format!("{}", Foo(23)), "23");
  // println!("1: {:#}", Foo {bar: 23});
  // println!("2: {}", Foo {bar: 24});
  // let combinations = Combinations::new(4);                          // Columns * rows

  // println!("Bits set in main: {}", bit_utils::number_of_bits_set(333)); // 333 = 101001101

  println!("======================================");

  it_creates_a_4x1_grid();

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
