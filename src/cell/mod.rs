pub mod cell;
mod cell_test;
mod json_cell_column;
mod json_cell_row;
mod json_cell;

pub const SYMBOLS: [char; 36] = [                                   // Max 6 x 6
  '1', '2', '3', '4', '5', '6',
  '7', '8', '9', 'A', 'B', 'C',
  'D', 'E', 'F', 'G', 'H', 'I',
  'J', 'K', 'L', 'M', 'N', 'O',
  'P', 'Q', 'R', 'S', 'T', 'U',
  'V', 'W', 'X', 'Y', 'Z', '0'
];

pub enum SetMethod {
  Unset,
	Loaded,
	User,
	Calculated
}