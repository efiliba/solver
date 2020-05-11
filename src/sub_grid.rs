use std::fmt::{self, Display};
use crate::cell::{cell::Cell, dimensions::Dimensions, SetMethod, SYMBOLS};

// use crate::utils::bit_utils::{highest_bit_position, number_of_bits_set, power_of_2_bit_positions};

#[derive(Debug)]
pub struct Option {
  pub sub_grid_column: usize,
  pub sub_grid_row: usize,
  pub cell_column: usize,
  pub cell_row: usize,
  pub bits: usize
}

impl Option {
  pub fn new(
    sub_grid_column: usize,
    sub_grid_row: usize,
    cell_column: usize,
    cell_row: usize,
    bits: usize
  ) -> Self {
    Option {
      sub_grid_column,
      sub_grid_row,
      cell_column,
      cell_row,
      bits
    }
  }
}

pub struct StruckOutCells {
  pub last_options_found: Vec<Option>,
  pub removed_options_from_column: Vec<Option>,
  pub removed_options_from_row: Vec<Option>
}

// impl StruckOutCells {
//   pub fn new(
//     last_options_found: Vec<Option>,
//     removed_options_from_column: Vec<Option>,
//     removed_options_from_row: Vec<Option>
//   ) -> Self {
//     StruckOutCells {
//       last_options_found,
//       removed_options_from_column,
//       removed_options_from_row
//     }
//   }
// }

#[derive(Debug)]
pub struct SubGrid<'a> {
  dimensions: &'a Dimensions,

  pub column: usize,
  pub row: usize,
  cells: Vec<Vec<Cell<'a>>>                                         // use get(column, row) -> returns cells[row][column]
}

impl Display for SubGrid<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut output = String::new();

    for row in 0..self.dimensions.rows {
      for column in 0..self.dimensions.columns {
        fmt::write(&mut output, format_args!("{} ", self.cells[row][column]))
          .expect("Error writing cell in sub-grid");
      }
    }

    write!(f, "{}", output)
  }
}


impl<'a> SubGrid<'a> {
  pub fn new(dimensions: &'a Dimensions, column: usize, row: usize) -> Self {
    let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(dimensions.rows);

    for row in 0..dimensions.rows {
      cells.push(Vec::with_capacity(dimensions.columns));
      let swopped = dimensions.swop();
      for column in 0..dimensions.columns {
        cells[row].push(Cell::new(swopped, column, row));
      }
    }
  
    SubGrid {
      dimensions,
      column,
      row,
      cells
    }
  }

  pub fn reset(&mut self) {
    for row in 0..self.dimensions.rows {
      self.cells.push(Vec::with_capacity(self.dimensions.columns));
      for column in 0..self.dimensions.columns {
        // self.cells[row][column] = Cell::new(&self.dimensions, column, row);
        self.cells[row][column].reset();
      }
    }
  }

  pub fn get(&self, column: usize, row: usize) -> &Cell {
    // grids called by [column, row] but accessed by [row][column] for efficiency
    &self.cells[row][column]
  }

  pub fn compare(&self, items: Vec<Vec<Cell>>) -> bool {
    let mut equal = true;
    let mut row = self.dimensions.rows;
    while equal && row > 0 {
      row -= 1;
      let mut column = self.dimensions.columns;
      while equal && column > 0 {
        column -= 1;
        equal = self.cells[row][column].equal(&items[row][column]);
      }
    }

    equal
  }

  pub fn set_by_position(
    &mut self,
    column: usize,
    row: usize,
    option_column: usize,
    option_row: usize,
    set_method: SetMethod
  ) -> bool {
    let cell = &mut self.cells[row][column];
    if cell.set_method == SetMethod::Unset {
      // cell unset i.e. == SetMethod.unset
      cell.set_by_position(option_column, option_row, set_method);
      return true;
    }
    false
  }

  pub fn set_by_option(
    &mut self,
    column: usize,
    row: usize,
    option: usize,
    set_method: SetMethod
  ) -> bool {
    let cell = &mut self.cells[row][column];
    if cell.set_method == SetMethod::Unset {
      cell.set_by_option(option, set_method);
      return true;
    }
    false
  }

  pub fn set_by_symbol(
    &mut self,
    column: usize,
    row: usize,
    symbol: char,
    set_method: SetMethod
  ) -> usize {
    let cell = &mut self.cells[row][column];
    if cell.set_method == SetMethod::Unset {
      cell.set_by_symbol(symbol, set_method);
      return cell.options;
    }
    0
  }

  pub fn solved(&self) -> bool {
    let mut solved = true;

    let mut row = self.dimensions.rows;
    while solved && row > 0 {
      row -= 1;
      let mut column = self.dimensions.columns;
      while solved && column > 0 {
        column -= 1;
        solved = self.cells[row][column].solved();
      }
    }

    solved
  }

  // Remove option from all other cells in this sub grid - return array of last options found and options removed from all columns / rows in the sub grid
  pub fn strike_out_cell(
    &mut self,
    cell_column: usize,
    cell_row: usize,
    option: usize
  ) -> StruckOutCells {
    let mut last_options: Vec<Option> = Vec::with_capacity(5);
    let mut removed_options_from_column: Vec<Option> = Vec::with_capacity(5);
    let mut removed_options_from_row: Vec<Option> = Vec::with_capacity(5);

    let mut column;
    let mut row = self.dimensions.rows - 1;
    while row > cell_row {
      column = self.dimensions.columns;
      while column > 0 {
        column -= 1;
        if self.cells[row][column].remove_option(option) {
          last_options.push(Option::new(
            self.column,
            self.row,
            column,
            row,
            self.cells[row][column].options,
          ));
        } else {
          if self.option_removed_from_column(column, row, option) {
            removed_options_from_column.push(Option::new(
              self.column,
              self.row,
              column,
              0, // ToDo: -1,
              option,
            ));
          }
          if self.option_removed_from_row(column, row, option) {
            removed_options_from_row.push(Option::new(
              self.column,
              self.row,
              0, // ToDo: -1,
              row,
              option,
            ));
          }
        }
      }
      row -= 1;
    }

    let mut column = self.dimensions.columns - 1;
    while column > cell_column {
      if self.cells[row][column].remove_option(option) {
        last_options.push(Option::new(
          self.column,
          self.row,
          column,
          row,
          self.cells[row][column].options
        ));
      } else {
        if self.option_removed_from_column(column, row, option) {
          removed_options_from_column.push(Option::new(
            self.column,
            self.row,
            column,
            0, // ToDo: -1,
            option,
          ));
        }
        if self.option_removed_from_row(column, row, option) {
          removed_options_from_row.push(Option::new(
            self.column,
            self.row,
            0, // ToDo: -1,
            row,
            option,
          ));
        }
      }
      column -= 1;
    }

    while column > 0 {
      column -= 1;
      if self.cells[row][column].remove_option(option) {
        last_options.push(Option::new(
            self.column,
            self.row,
            column,
            row,
            self.cells[row][column].options
          ));
      } else {
        if self.option_removed_from_column(column, row, option) {
          removed_options_from_column.push(Option::new(
            self.column,
            self.row,
            column,
            0, // ToDo: -1,
            option,
          ));
        }
        if self.option_removed_from_row(column, row, option) {
          removed_options_from_row.push(Option::new(
            self.column,
            self.row,
            0, // ToDo: -1,
            0, // ToDo: -1,
            option,
          ));
        }
      }
    }

    while row > 0 {
      row -= 1;
      column = self.dimensions.columns;
      while column > 0 {
        column -= 1;
        if self.cells[row][column].remove_option(option) {
          last_options.push(Option::new(
            self.column,
            self.row,
            column,
            row,
            self.cells[row][column].options,
          ));
        } else {
          if self.option_removed_from_column(column, row, option) {
            removed_options_from_column.push(Option::new(
              self.column,
              self.row,
              column,
              0, // ToDo: -1,
              option,
            ));
          }
          if self.option_removed_from_row(column, row, option) {
            removed_options_from_row.push(Option::new(
              self.column,
              self.row,
              0, // ToDo: -1,
              row,
              option,
            ));
          }
        }
      }
    }

    StruckOutCells {
      last_options_found: last_options,
      removed_options_from_column,
      removed_options_from_row
    }
  }


  pub fn option_removed_from_column(
    &self,
    cell_column: usize,
    cell_row: usize,
    option: usize
  ) -> bool {
    // Check if option removed from column
    let mut option_found = false;
    let mut row = self.dimensions.rows - 1;
    while !option_found && row > cell_row {
      option_found = (self.cells[row][cell_column].options & option) > 0;
      row -= 1;
    }
    while !option_found && row > 0 {
      row -= 1;
      option_found = (self.cells[row][cell_column].options & option) > 0;
    }

    !option_found                                                   // If option not found then it was removed from this sub grid's column
  }

  pub fn option_removed_from_row(
    &self,
    cell_column: usize,
    cell_row: usize,
    removed_option: usize
  ) -> bool {
    // Check if option removed from row
    let mut option_found = false;
    let mut column = self.dimensions.columns - 1;
    while !option_found && column > cell_column {
      option_found = (self.cells[cell_row][column].options & removed_option) > 0;
      column -= 1;
    }
    while !option_found && column > 0 {
      option_found = (self.cells[cell_row][column].options & removed_option) > 0;
      column -= 1;
    }

    return !option_found;                                           // If option not found then it was removed from this sub grid's row
  }

  pub fn set_cells(&self, sub_grid: Vec<Vec<Cell>>) {
    for row in 0..self.dimensions.rows {
      for column in 0..self.dimensions.columns {
        // self.cells[row][column] = Cell::new(sub_grid[row][column]);
      }
    }
  }

}
