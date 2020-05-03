use super::json_cell_row::JsonCellRow;

#[derive(Debug)]
pub struct JsonCell {
	rows: Vec<JsonCellRow>,
	// symbol: String,
	// set_method: SetMethod
}

impl JsonCell {
  pub fn new(columns: usize, rows: usize) -> JsonCell {
		let mut json_rows: Vec<JsonCellRow> = Vec::with_capacity(rows);

    for row in 0..rows {
      json_rows.push(JsonCellRow::new(columns, row));
    }

    JsonCell {
      rows: json_rows,
      // symbol: String,
      // set_method: SetMethod::Unset
    }
  }

  pub fn print(&mut self) {
      println!("JsonCell {:#?}", self);
  }
}