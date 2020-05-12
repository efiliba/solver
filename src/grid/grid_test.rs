#[cfg(test)]
use crate::cell::{cell::Cell, dimensions::Dimensions};
use crate::sub_grid::sub_grid::SubGrid;

#[cfg(test)]
fn init_sub_grids(dimensions: &Dimensions) -> Vec<Vec<SubGrid>> {
  let mut sub_grids: Vec<Vec<SubGrid>> = Vec::with_capacity(dimensions.total);
  let swopped = dimensions.swop();
  for row in 0..dimensions.rows {
    sub_grids.push(Vec::with_capacity(dimensions.columns));
    for column in 0..dimensions.columns {
      sub_grids[row].push(SubGrid::new(swopped, column, row));
    }
  }

  sub_grids
}

#[cfg(test)]
mod grid {
  use crate::cell::{dimensions::Dimensions, SetMethod};
  use crate::grid::grid::Grid;

  #[test]
  fn it_creates_a_4x1_grid() {
    let columns = 4;
    let rows = 1;
    let dimensions = Dimensions::new(columns, rows);
    let grid = Grid::new(&dimensions);

    // Ensure a 2 x 4 sub grid created
    let expected_sub_grids = super::init_sub_grids(&dimensions);
    // assert!(grid.compare(&expected_sub_grids));

    // println!("{:#?}", grid);
    // assert!(false);
  }

}