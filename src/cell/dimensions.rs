#[derive(Debug)]
pub struct Dimensions {
  pub columns: usize,
  pub rows: usize,
  pub total: usize
}

impl Dimensions {
  pub fn new(columns: usize, rows: usize) -> Dimensions {
    Dimensions {
      columns,
      rows,
      total: columns * rows
    }
  }
}
