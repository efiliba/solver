#[cfg(test)]
mod sub_grid_2x4 {
  use crate::cell::{cell::Cell, dimensions::Dimensions, SetMethod};
  use crate::sub_grid::SubGrid;

  fn init_cells(dimensions: &Dimensions) -> Vec<Vec<Cell>> {
    let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(dimensions.total);
    let swopped = dimensions.swop();
    for row in 0..dimensions.rows {
      cells.push(Vec::with_capacity(dimensions.columns));
      for column in 0..dimensions.columns {
        cells[row].push(Cell::new(swopped, column, row));
      }
    }

    cells
  }

  #[test]
  fn it_creates_a_2x4_sub_grid() {
    let columns = 2;
    let rows = 4;
    let dimensions = Dimensions::new(columns, rows);
    let sub_grid = SubGrid::new(&dimensions, 0, 0);

    // Ensure a 2 x 4 sub grid created
    let expected_cells = init_cells(&dimensions);
    assert!(sub_grid.compare(expected_cells));
  }

  #[test]
  fn it_creates_a_4x2_sub_grid() {
    let columns = 4;
    let rows = 2;
    let dimensions = Dimensions::new(columns, rows);
    let sub_grid = SubGrid::new(&dimensions, 1, 3);                 // Bottom right Cell of parent grid          

    let expected_cells = init_cells(&dimensions);
    assert!(sub_grid.compare(expected_cells));

    assert_eq!(sub_grid.get(0, 0).options, 255);
    assert_eq!(sub_grid.get(1, 0).options, 255);
    assert_eq!(sub_grid.get(2, 0).options, 255);
    assert_eq!(sub_grid.get(3, 0).options, 255);
    assert_eq!(sub_grid.get(0, 1).options, 255);
    assert_eq!(sub_grid.get(1, 1).options, 255);
    assert_eq!(sub_grid.get(2, 1).options, 255);
    assert_eq!(sub_grid.get(3, 1).options, 255);
  }

  #[test]
  fn it_solves_the_4x2_sub_grid() {
    let dimensions = Dimensions::new(4, 2);
    let mut sub_grid = SubGrid::new(&dimensions, 1, 3);

    sub_grid.set_by_position(0, 0, 0, 0, SetMethod::User);
    sub_grid.set_by_option(1, 0, 2, SetMethod::User);
    sub_grid.set_by_position(2, 0, 2, 0, SetMethod::Loaded);
    sub_grid.set_by_option(3, 0, 4, SetMethod::Loaded);
    sub_grid.set_by_option(0, 1, 8, SetMethod::Loaded);
    sub_grid.set_by_position(1, 1, 1, 1, SetMethod::Loaded);
    sub_grid.set_by_option(2, 1, 32, SetMethod::User);
    sub_grid.set_by_position(3, 1, 3, 1, SetMethod::User);

    assert!(sub_grid.solved());
  }

  #[test]
  fn it_removes_1_from_all_cells_except_top_left_in_3x2_sub_grid() {
    let dimensions = Dimensions::new(3, 2);
    let mut sub_grid = SubGrid::new(&dimensions, 0, 0);

    // Remove 1 from all cells except top left cell - check 1 removed from other cells
    let remove_bit = 1;
    let test_column = 0;
    let test_row = 0;
  
    let struck_out_cells = sub_grid.strike_out_cell(test_column, test_row, remove_bit);
    assert_eq!(struck_out_cells.last_options_found.len(), 0);       // No last options found
  
    let remove_from_column = struck_out_cells.removed_options_from_column;  // Removed 1 from columns 2 and 1
    assert_eq!(remove_from_column.len(), 2);
    assert_eq!(remove_from_column[0].cell_column, 2);
    assert_eq!(remove_from_column[0].bits, remove_bit);
    assert_eq!(remove_from_column[1].cell_column, 1);
    assert_eq!(remove_from_column[1].bits, remove_bit);

    let remove_from_row = struck_out_cells.removed_options_from_row;// Removed 1 from row 1
    assert_eq!(remove_from_row.len(), 1);
    assert_eq!(remove_from_row[0].cell_row, remove_bit);
    assert_eq!(remove_from_column[0].bits, remove_bit);

    let mut expected_cells = init_cells(&dimensions);

    for row in 0..dimensions.rows {
      for column in 0..dimensions.columns {
        expected_cells[row][column].remove_option(remove_bit);
      }
    }
    expected_cells[test_row][test_column].reset();                  // No options removed
    assert!(sub_grid.compare(expected_cells));
  }

  #[test]
  fn it_also_removes_2_from_all_cells_except_top_middle() {
    let dimensions = Dimensions::new(3, 2);
    let mut sub_grid = SubGrid::new(&dimensions, 0, 0);

    sub_grid.strike_out_cell(0, 0, 1);                              // Continue from previous test

    // Remove 2 from all cells except top left cell - check 2 removed from other cells
    let remove_bit = 2;
    let test_column = 0;
    let test_row = 0;
    
    let struck_out_cells = sub_grid.strike_out_cell(test_column, test_row, remove_bit);
    assert_eq!(struck_out_cells.last_options_found.len(), 0);       // No last options found
    
    let remove_from_column = struck_out_cells.removed_options_from_column;  // Removed 2 from columns 2 and 1
    assert_eq!(remove_from_column.len(), 2);
    assert_eq!(remove_from_column[0].cell_column, 2);
    assert_eq!(remove_from_column[0].bits, remove_bit);
    assert_eq!(remove_from_column[1].cell_column, 1);
    assert_eq!(remove_from_column[1].bits, remove_bit);
    
    let remove_from_row = struck_out_cells.removed_options_from_row;  // Removed 2 from row 1
    assert_eq!(remove_from_row.len(), 1);
    assert_eq!(remove_from_row[0].cell_row, 1);
    assert_eq!(remove_from_column[0].bits, remove_bit);

    let mut expected_cells = init_cells(&dimensions);
      
    for row in 0..dimensions.rows {
      for column in 0..dimensions.columns {
        expected_cells[row][column].remove_option(1);               // Continue from previous test
        expected_cells[row][column].remove_option(remove_bit);
      }
    }
    expected_cells[test_row][test_column].reset();                  // No options removed
    assert!(sub_grid.compare(expected_cells));
  }







    
  #[test]
  fn it_removes_16_from_all_cells_except_bottom_middle() {
    let dimensions = Dimensions::new(3, 2);
    let mut sub_grid = SubGrid::new(&dimensions, 0, 0);

    // Remove 16 from bottom middle cell
    let remove_bit = 16;
    let test_column = 1;
    let test_row = 1;
  
    let struck_out_cells = sub_grid.strike_out_cell(test_column, test_row, remove_bit);
    let last_options = struck_out_cells.last_options_found;
    // assert_eq!(last_options.len(), 1);                              // a last option was found
    // assert_eq!(last_options[0].cell_column, 2);                     // (2, 1) must be 6
    // assert_eq!(last_options[0].cell_row, 1);
    // assert_eq!(last_options[0].bits, 32);

  }
      



}
