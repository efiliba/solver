use std::fmt::{self, Display};
use crate::cell::{cell::Cell, dimensions::Dimensions, SetMethod, SYMBOLS};
use crate::sub_grid::{BitOption, StruckOutCells};

#[derive(Debug)]
pub struct SubGrid<'a> {
  dimensions: &'a Dimensions,

  pub column: usize,
  pub row: usize,
  cells: Vec<Vec<Cell<'a>>>                                         // use get(column, row) -> returns cells[row][column]
}

impl Display for SubGrid<'_> {
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
        fmt::write(&mut output, format_args!("{} ", self.cells[row][column]))
          .expect("Error writing cell in sub-grid");
      }
      add_new_line(&mut output);
    }
    add_separator_line(&mut output, self.dimensions);

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
      for column in 0..self.dimensions.columns {
        self.cells[row][column].reset();
      }
    }
  }

  pub fn get(&self, column: usize, row: usize) -> &Cell {
    // grids called by [column, row] but accessed by [row][column] for efficiency
    &self.cells[row][column]
  }

  pub fn compare(&self, items: &Vec<Vec<Cell>>) -> bool {
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

  pub fn compare_ref(&self, items: &Vec<Vec<&Cell>>) -> bool {
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

  pub fn simplify(&mut self) {
    let mut changed = true;
    while changed {
      changed = false;

      let mut row = self.dimensions.rows;
      while !changed && row > 0 {
        row -= 1;
        let mut column = self.dimensions.columns;
        while !changed && column > 0 {
          column -= 1;
          changed =
            self.cells[row][column].set_method != SetMethod::Unset && // cell set
            self.remove_if_extra_options(self.cells[row][column].options).len() > 0;
        }
      }
    }
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

  pub fn get_cells_matrix(&self) -> Vec<Vec<&Cell>> {
    let mut matrix: Vec<Vec<&Cell>> = Vec::with_capacity(self.dimensions.rows);

    for row in 0..self.dimensions.rows {
      matrix.push(Vec::with_capacity(self.dimensions.columns));

      for column in 0..self.dimensions.columns {
        matrix[row].push(&self.cells[row][column]);
      }
    }

    matrix
  }

  // Remove option from all other cells in this sub grid - return array of last options found and options removed from all columns / rows in the sub grid
  pub fn strike_out_cell(
    &mut self,
    cell_column: usize,
    cell_row: usize,
    option: usize
  ) -> StruckOutCells {
    let mut last_options: Vec<BitOption> = Vec::new();
    let mut removed_options_from_column: Vec<BitOption> = Vec::new();
    let mut removed_options_from_row: Vec<BitOption> = Vec::new();

    let mut column;
    let mut row = self.dimensions.rows - 1;
    while row > cell_row {
      column = self.dimensions.columns;
      while column > 0 {
        column -= 1;
        if self.cells[row][column].remove_option(option) {
          last_options.push(BitOption {
            sub_grid_column: self.column,
            sub_grid_row: self.row,
            cell_column: column,
            cell_row: row,
            bits: self.cells[row][column].options
          });
        } else {
          if self.option_removed_from_column(column, row, option) {
            removed_options_from_column.push(BitOption {
              sub_grid_column: self.column,
              sub_grid_row: self.row,
              cell_column: column,
              cell_row: 0, // ToDo: -1,
              bits: option
            });
          }
          if self.option_removed_from_row(column, row, option) {
            removed_options_from_row.push(BitOption {
              sub_grid_column: self.column,
              sub_grid_row: self.row,
              cell_column: 0, // ToDo: -1,
              cell_row: row,
              bits: option
            });
          }
        }
      }
      row -= 1;
    }

    let mut column = self.dimensions.columns - 1;
    while column > cell_column {
      if self.cells[row][column].remove_option(option) {
        last_options.push(BitOption {
          sub_grid_column: self.column,
          sub_grid_row: self.row,
          cell_column: column,
          cell_row: row,
          bits: self.cells[row][column].options
        });
      } else {
        if self.option_removed_from_column(column, row, option) {
          removed_options_from_column.push(BitOption {
            sub_grid_column: self.column,
            sub_grid_row: self.row,
            cell_column: column,
            cell_row: 0, // ToDo: -1,
            bits: option
          });
        }
        if self.option_removed_from_row(column, row, option) {
          removed_options_from_row.push(BitOption {
            sub_grid_column: self.column,
            sub_grid_row: self.row,
            cell_column: 0, // ToDo: -1,
            cell_row: row,
            bits: option
          });
        }
      }
      column -= 1;
    }

    while column > 0 {
      column -= 1;
      if self.cells[row][column].remove_option(option) {
        last_options.push(BitOption {
          sub_grid_column: self.column,
          sub_grid_row: self.row,
          cell_column: column,
          cell_row: row,
          bits: self.cells[row][column].options
        });
      } else {
        if self.option_removed_from_column(column, row, option) {
          removed_options_from_column.push(BitOption {
            sub_grid_column: self.column,
            sub_grid_row: self.row,
            cell_column: column,
            cell_row: 0, // ToDo: -1,
            bits: option
          });
        }
        if self.option_removed_from_row(column, row, option) {
          removed_options_from_row.push(BitOption {
            sub_grid_column: self.column,
            sub_grid_row: self.row,
            cell_column: 0, // ToDo: -1,
            cell_row: 0, // ToDo: -1,
            bits: option
          });
        }
      }
    }

    while row > 0 {
      row -= 1;
      column = self.dimensions.columns;
      while column > 0 {
        column -= 1;
        if self.cells[row][column].remove_option(option) {
          last_options.push(BitOption {
            sub_grid_column: self.column,
            sub_grid_row: self.row,
            cell_column: column,
            cell_row: row,
            bits: self.cells[row][column].options
          });
        } else {
          if self.option_removed_from_column(column, row, option) {
            removed_options_from_column.push(BitOption {
              sub_grid_column: self.column,
              sub_grid_row: self.row,
              cell_column: column,
              cell_row: 0, // ToDo: -1,
              bits: option
            });
          }
          if self.option_removed_from_row(column, row, option) {
            removed_options_from_row.push(BitOption {
              sub_grid_column: self.column,
              sub_grid_row: self.row,
              cell_column: 0, // ToDo: -1,
              cell_row: row,
              bits: option
            });
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
    let mut row = self.dimensions.rows;
    while !option_found && row > cell_row + 1 {
      row -= 1;
      option_found = (self.cells[row][cell_column].options & option) > 0;
    }

    row -= 1;                                                       // Skip row_column
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
    let mut column = self.dimensions.columns;
    while !option_found && column > cell_column + 1 {
      column -= 1;
      option_found = (self.cells[cell_row][column].options & removed_option) > 0;
    }
    
    column -= 1;                                                    // Skip cell_column
    while !option_found && column > 0 {
      column -= 1;
      option_found = (self.cells[cell_row][column].options & removed_option) > 0;
    }

    !option_found                                                   // If option not found then it was removed from this sub grid's row
  }


  // public removeOptionsExceptFromColumn(
  //   excludeColumn: number,
  //   options: number
  // ): IOption[] {
  //   let last_options: IOption[] = [];

  //   let row: number;
  //   let column: number = self.dimensions.columns;
  //   while (--column > excludeColumn) {
  //     row = self.dimensions.rows;
  //     while (row--) {
  //       if (self.cells[row][column].removeOptions(options)) {
  //         last_options.push({
  //           sub_grid_column: self.column,
  //           sub_grid_row: self.row,
  //           cell_column: column,
  //           cell_row: row,
  //           bits: self.cells[row][column].options,
  //         });
  //         //                        self.remainingCells--;
  //       }
  //     }
  //   }

  //   while (column--) {
  //     row = self.dimensions.rows;
  //     while (row--) {
  //       if (self.cells[row][column].removeOptions(options)) {
  //         last_options.push({
  //           sub_grid_column: self.column,
  //           sub_grid_row: self.row,
  //           cell_column: column,
  //           cell_row: row,
  //           bits: self.cells[row][column].options,
  //         });
  //         //                        self.remainingCells--;
  //       }
  //     }
  //   }

  //   return last_options;
  // }

  // public removeOptionsExceptFromRow(
  //   excludeRow: number,
  //   options: number
  // ): IOption[] {
  //   let last_options: IOption[] = [];

  //   let column: number;
  //   let row: number = self.dimensions.rows;
  //   while (--row > excludeRow) {
  //     column = self.dimensions.columns;
  //     while (column--) {
  //       if (self.cells[row][column].removeOptions(options)) {
  //         last_options.push({
  //           sub_grid_column: self.column,
  //           sub_grid_row: self.row,
  //           cell_column: column,
  //           cell_row: row,
  //           bits: self.cells[row][column].options,
  //         });
  //         //                        self.remainingCells--;
  //       }
  //     }
  //   }

  //   while (row--) {
  //     column = self.dimensions.columns;
  //     while (column--) {
  //       if (self.cells[row][column].removeOptions(options)) {
  //         last_options.push({
  //           sub_grid_column: self.column,
  //           sub_grid_row: self.row,
  //           cell_column: column,
  //           cell_row: row,
  //           bits: self.cells[row][column].options,
  //         });
  //         //                        self.remainingCells--;
  //       }
  //     }
  //   }

  //   return last_options;
  // }

  // pub fn remove_if_extra_options_from_column(
  //   column: usize,
  //   options: usize
  // ) -> Vec<BitOption> {
  //   let last_options: IOption[] = [];

  //   for (let row: number = 0; row < self.dimensions.rows; row++) {
  //     if (self.cells[row][column].removeOptions(options)) {
  //       last_options.push({
  //         sub_grid_column: self.column,
  //         sub_grid_row: self.row,
  //         cell_column: column,
  //         cell_row: row,
  //         bits: self.cells[row][column].options,
  //       });
  //       //                    self.remainingCells--;
  //     }
  //   }

  //   return last_options;
  // }

  // public removeIfExtraOptionsFromRow(row: number, options: number): IOption[] {
  //   let last_options: IOption[] = [];

  //   for (let column: number = 0; column < self.dimensions.columns; column++) {
  //     if (self.cells[row][column].removeOptions(options)) {
  //       last_options.push({
  //         sub_grid_column: self.column,
  //         sub_grid_row: self.row,
  //         cell_column: column,
  //         cell_row: row,
  //         bits: self.cells[row][column].options,
  //       });
  //       //                    self.remainingCells--;
  //     }
  //   }

  //   return last_options;
  // }

  pub fn remove_if_extra_options(&mut self, options: usize) -> Vec<BitOption> {
    let mut last_options = Vec::new();

    for row in 0..self.dimensions.rows {
      for column in 0..self.dimensions.columns {
        if self.cells[row][column].remove_options(options) {
          last_options.push(BitOption {
            sub_grid_column: self.column,
            sub_grid_row: self.row,
            cell_column: column,
            cell_row: row,
            bits: self.cells[row][column].options,
          });
        }
      }
    }

    return last_options;
  }

  // public optionExistsInColumn(column: number, option: number): boolean {
  //   let found: boolean = false;
  //   let row: number = self.dimensions.rows;
  //   while (!found && row--) {
  //     found = self.cells[row][column].containsOption(option);
  //   }

  //   return found;
  // }

  // public optionExistsInRow(row: number, option: number): boolean {
  //   let found: boolean = false;
  //   let column: number = self.dimensions.columns;
  //   while (!found && column-- > 0) {
  //     found = self.cells[row][column].containsOption(option);
  //   }

  //   return found;
  // }

  // public optionRemovedFromColumn(
  //   cell_column: number,
  //   cell_row: number,
  //   option: number
  // ): boolean {
  //   // Check if option removed from column
  //   let optionFound: boolean = false;

  //   let row: number = self.dimensions.rows;
  //   while (!optionFound && --row > cell_row) {
  //     optionFound = (self.cells[row][cell_column].options & option) > 0;
  //   }
  //   while (!optionFound && row--) {
  //     optionFound = (self.cells[row][cell_column].options & option) > 0;
  //   }

  //   return !optionFound; // If option not found then it was removed from self sub grid's column
  // }

  // public optionRemovedFromRow(
  //   cell_column: number,
  //   cell_row: number,
  //   removedOption: number
  // ): boolean {
  //   // Check if option removed from row
  //   let optionFound: boolean = false;
  //   let column: number = self.dimensions.columns;
  //   while (!optionFound && --column > cell_column) {
  //     optionFound = (self.cells[cell_row][column].options & removedOption) > 0;
  //   }
  //   while (!optionFound && column--) {
  //     optionFound = (self.cells[cell_row][column].options & removedOption) > 0;
  //   }

  //   return !optionFound; // If option not found then it was removed from self sub grid's row
  // }


  pub fn set_cells(&self, sub_grid: Vec<Vec<Cell>>) {
    for row in 0..self.dimensions.rows {
      for column in 0..self.dimensions.columns {
        // self.cells[row][column] = Cell::new(sub_grid[row][column]); -> set using copy letructor ?
      }
    }
  }
}
