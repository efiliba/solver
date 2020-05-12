use std::fmt::{self, Display};
use crate::utils::bit_utils::{number_of_bits_set, only_option, containing_bit_index};
use crate::cell::{cell::Cell, dimensions::Dimensions, SetMethod, SYMBOLS};
use crate::sub_grid::{sub_grid::SubGrid, BitOption, StruckOutCells};

// import {IOnlyOption, bitwiseOR, numberOfBitsSet, onlyOption, containingBitIndex} from "./utils/bitUtilities";
// import {Combinations} from "./utils/combinations";
// import {SetMethod, ICell, IJsonCell} from "./cell";
// import {SubGrid, ISubGrid, IOption, DebugSubGridType, IStruckOutCells, IJsonSubGrid} from "./subGrid";

#[derive(Debug)]
pub struct Grid<'a> {
  dimensions: &'a Dimensions,
  // combinations: Combinations<Cell>,
  
  sub_grids: Vec<Vec<SubGrid<'a>>>,                                 // use get(column, row) -> returns sub-grids[row][column]
  total_set: usize
}
  
impl Display for Grid<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fn add_separator_line(output: &mut String, dimensions: &Dimensions) {
      // All left aligned padding '-' to ''
      fmt::write(output, format_args!("{:-<1$}", "", (dimensions.total + 1) * dimensions.columns - 1))
        .expect("Error adding separator line in sub-grid");
    }

    fn add_new_line(output: &mut String) {
      fmt::write(output, format_args!("\n")).expect("Error adding new line in sub-grid");
    }

    let mut output = String::new();
    add_separator_line(&mut output, self.dimensions);
    add_new_line(&mut output);

    for row in 0..self.dimensions.rows {
      for column in 0..self.dimensions.columns {
        fmt::write(&mut output, format_args!("{} ", self.sub_grids[row][column]))
          .expect("Error writing cell in sub-grid");
      }
      add_new_line(&mut output);
    }
    add_separator_line(&mut output, self.dimensions);

    write!(f, "{}", output)
  }
}

impl<'a> Grid<'a> {
  pub fn new(dimensions: &'a Dimensions) -> Self {
    let mut sub_grids: Vec<Vec<SubGrid>> = Vec::with_capacity(dimensions.rows);

    for row in 0..dimensions.rows {
      sub_grids.push(Vec::with_capacity(dimensions.columns));
      let swopped = dimensions.swop();
      for column in 0..dimensions.columns {
        sub_grids[row].push(SubGrid::new(swopped, column, row));
      }
    }
  
    Grid {
      dimensions,
      sub_grids,
      total_set: 0
    }
  }

  pub fn reset(&mut self) {
    for row in 0..self.dimensions.rows {
      for column in 0..self.dimensions.columns {
        self.sub_grids[row][column].reset();
      }
    }
    self.total_set = 0;
  }

  pub fn get(&self, column: usize, row: usize) -> &SubGrid {
    // sub-grids called by [column, row] but accessed by [row][column] for efficiency
    &self.sub_grids[row][column]
  } 
  
  pub fn compare(&self, items: &Vec<Vec<SubGrid>>) -> bool {
    let mut equal = true;
    let mut row = self.dimensions.rows;
    while equal && row > 0 {
      row -= 1;
      let mut column = self.dimensions.columns;
      while equal && column > 0 {
        column -= 1;
        let matrix = items[row][column].get_cells_matrix();
        equal = self.sub_grids[row][column].compare_ref(&matrix);
      }
    }

    equal
  }
}