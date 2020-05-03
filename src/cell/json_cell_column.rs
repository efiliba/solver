use crate::cell::SYMBOLS;

#[derive(Debug)]
pub struct JsonCellColumn {
	symbol: char,
	strike_out: bool,
	highlight: bool
}

impl JsonCellColumn {
  pub fn new(index: usize) -> JsonCellColumn {
    JsonCellColumn {
      symbol: SYMBOLS[index],
      strike_out: false,
      highlight: false
    }
  }
}
