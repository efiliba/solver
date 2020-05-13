use std::fmt::{self, Display};
use crate::utils::bit_utils::{number_of_bits_set, only_option, containing_bit_index};
use crate::cell::{cell::Cell, dimensions::Dimensions, SetMethod, SYMBOLS};
use crate::sub_grid::{sub_grid::SubGrid, BitOption, StruckOutCells};

// import {IOnlyOption, bitwiseOR, numberOfBitsSet, onlyOption, containingBitIndex} from "./utils/bitUtilities";
// import {Combinations} from "./utils/combinations";
// import {SetMethod, ICell, IJsonCell} from "./cell";
// import {SubGrid, ISubGrid, BitOption, DebugSubGridType, IStruckOutCells, IJsonSubGrid} from "./subGrid";

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

	pub fn strike_out(
    &mut self,
    sub_grid_column: usize,
    sub_grid_row: usize,
    cell_column: usize,
    cell_row: usize,
    option: usize
  ) {
		let mut struck_out_cells = self.sub_grids[sub_grid_row][sub_grid_column]
			.strike_out_cell(cell_column, cell_row, option);

		let mut remove_option: &BitOption;

		let mut index = struck_out_cells.removed_options_from_column.len(); // Distinct
		while index > 0 {
      index -= 1;
			remove_option = &struck_out_cells.removed_options_from_column[index];
			struck_out_cells.last_options_found.append(
				&mut self.remove_option_from_other_columns(
					remove_option.sub_grid_column,
					remove_option.sub_grid_row,
					remove_option.cell_column,
					remove_option.bits
				)
			);
		}

		// index = struck_out_cells.removed_options_from_row.len();
		// while index > 0 { 
    //   index -= 1;
		// 	remove_option = struck_out_cells.removed_options_from_row[index];
		// 	self.join(struck_out_cells.last_options_found, self.remove_option_from_other_rows(remove_option.sub_grid_column, remove_option.sub_grid_row, remove_option.cell_row, remove_option.bits));
		// }

		// self.join(struck_out_cells.last_options_found, self.remove_options_from_column(sub_grid_column, sub_grid_row, cell_column, option));
		// self.join(struck_out_cells.last_options_found, self.remove_options_from_row(sub_grid_column, sub_grid_row, cell_row, option));

		// let last_option: BitOption;
		// index = struck_out_cells.last_options_found.len();
		// while index > 0 {
    //   index -= 1;
		// 	last_option = struck_out_cells.last_options_found[index];
		// 	self.strike_out(last_option.sub_grid_column, last_option.sub_grid_row, last_option.cell_column, last_option.cell_row, last_option.bits);
		// }

		self.total_set += struck_out_cells.last_options_found.len();
  }
  
  pub fn set_by_option(
    &mut self,
    sub_grid_column: usize,
    sub_grid_row: usize,
    cell_column: usize,
    cell_row: usize,
    option: usize,
    set_method: SetMethod
  ) {
		if self.sub_grids[sub_grid_row][sub_grid_column].set_by_option(cell_column, cell_row, option, set_method) {
			self.total_set += 1;
		}

		self.strike_out(sub_grid_column, sub_grid_row, cell_column, cell_row, option);
  }
	
	
  fn remove_options_from_column(
		&mut self,
    sub_grid_column: usize,
    sub_grid_row: usize,
    cell_column: usize,
    options: usize
  ) -> Vec<BitOption> {
    let mut last_options = Vec::new();

    // Ignore sub_grid_row
    let mut row = self.dimensions.rows - 1;
    while row > sub_grid_row {
			last_options.append(
				&mut self.sub_grids[row][sub_grid_column].remove_options_from_column(
					cell_column,
					options
				)
			);
			row -= 1;
    }
    while row > 0 {
			row -= 1;
			last_options.append(
				&mut self.sub_grids[row][sub_grid_column].remove_options_from_column(
					cell_column,
					options
				)
			);
    }

    last_options
  }

  fn remove_options_from_row(
		&mut self,
    sub_grid_column: usize,
    sub_grid_row: usize,
    cellRow: usize,
    options: usize
  ) -> Vec<BitOption> {
    let mut last_options = Vec::new();

    // Ignore sub_grid_column
    let mut column = self.dimensions.columns - 1;
    while column > sub_grid_column {
			last_options.append(&mut self.sub_grids[sub_grid_row][column].remove_options_from_row(cellRow, options));
			column -= 1;
    }

    while column > 0 {
			column -= 1;
			last_options.append(&mut self.sub_grids[sub_grid_row][column].remove_options_from_row(cellRow, options));
    }

    last_options
	}
	
  // Check options removed from other columns (n - 1) columns must have the options removed i.e. option must exist in only 1 column
	fn remove_option_from_other_columns(
    &mut self,
    sub_grid_column: usize,
    sub_grid_row: usize,
    cell_column: usize,
    option: usize
  ) -> Vec<BitOption> {
		let mut last_options = Vec::new();

		let mut total_existing_columns = 0;
		let mut total_existing_rows = 0;

		let mut existing_column = 0; // ToDo: -1;
		let mut column = &self.dimensions.rows - 1;                         // Use SubGrid's number of columns i.e. swopped rows
		while total_existing_columns < 2 && column > cell_column {
      if self.sub_grids[sub_grid_row][sub_grid_column].option_exists_in_column(column, option) {
        existing_column = column;
				total_existing_columns += 1;
			}
      column -= 1;
		}
		while total_existing_columns < 2 && column > 0 {
      column -= 1;
			if self.sub_grids[sub_grid_row][sub_grid_column].option_exists_in_column(column, option) {
				existing_column = column;
				total_existing_columns += 1;
			}
		}

		if total_existing_columns == 1 {
				last_options = self.remove_options_from_column(sub_grid_column, sub_grid_row, existing_column, option);
		} else {
			// Check other sub grids in same column
			let mut existing_row = 0; // ToDo: -1;
			let mut row = &self.dimensions.rows - 1;
			while total_existing_rows < 2 && row > sub_grid_row {
				if self.sub_grids[row][sub_grid_column].option_exists_in_column(cell_column, option) {
					existing_row = row;
          total_existing_rows += 1;
          row -= 1;
				}
			}
			while total_existing_rows < 2 && row > 0 {
        row -= 0;
				if self.sub_grids[row][sub_grid_column].option_exists_in_column(cell_column, option) {
					existing_row = row;
					total_existing_rows += 1;
				}
			}

			if total_existing_rows == 1 {
				last_options = self.sub_grids[existing_row][sub_grid_column].remove_options_except_from_column(cell_column, option);
			}
		}

		last_options
	}

	// // Check options removed from other rows (n - 1) rows must have the options removed i.e. option must exist in only 1 row
	// fn removeOptionFromOtherRows(sub_grid_column: usize, sub_grid_row: usize, cellRow: usize, option: usize): BitOption[] {
	// 	let last_options: BitOption[] = [];

	// 	let total_existing_columns: usize = 0;
	// 	let total_existing_rows: usize = 0;

	// 	let existing_row: usize = -1;
	// 	let row: usize = Grid.columns;                              		// Use SubGrid's number of rows i.e. swopped columns
	// 	while (total_existing_rows < 2 && --row > cellRow) {
	// 		if (self.sub_grids[sub_grid_row][sub_grid_column].optionExistsInRow(row, option)) {
	// 			existing_row = row;
	// 			total_existing_rows++;
	// 		}
	// 	}
	// 	while (total_existing_rows < 2 && row-- > 0) {
	// 		if (self.sub_grids[sub_grid_row][sub_grid_column].optionExistsInRow(row, option)) {
	// 			existing_row = row;
	// 			total_existing_rows++;
	// 		}
	// 	}

	// 	if (total_existing_rows === 1) {
	// 			last_options = self.remove_options_from_Row(sub_grid_column, sub_grid_row, existing_row, option);
	// 	} else {
	// 		// Check other sub grids in same row
	// 		let existing_column: usize = -1;
	// 		let column: usize = Grid.columns;
	// 		while (total_existing_columns < 2 && --column > sub_grid_column) {
	// 			if (self.sub_grids[sub_grid_row][column].optionExistsInRow(cellRow, option)) {
	// 				existing_column = column;
	// 				total_existing_columns++;
	// 			}
	// 		}
	// 		while (total_existing_columns < 2 && column-- > 0) {
	// 			if (self.sub_grids[sub_grid_row][column].optionExistsInRow(cellRow, option)) {
	// 				existing_column = column;
	// 				total_existing_columns++;
	// 			}
	// 		}

	// 		if (total_existing_columns === 1) {
	// 			last_options = self.sub_grids[sub_grid_row][existing_column].removeOptionsExceptFromRow(cellRow, option);
	// 		}
	// 	}

	// 	return last_options;
	// }
}
