pub mod sub_grid;

mod sub_grid_test;

#[derive(Debug)]
pub struct Option {
  pub sub_grid_column: usize,
  pub sub_grid_row: usize,
  pub cell_column: usize,
  pub cell_row: usize,
  pub bits: usize
}

pub struct StruckOutCells {
  pub last_options_found: Vec<Option>,
  pub removed_options_from_column: Vec<Option>,
  pub removed_options_from_row: Vec<Option>
}
