#[cfg(test)]
mod cell {
  use crate::cell::{cell::Cell, dimensions::Dimensions};

  #[test]
  fn it_contains_option_at_position() {
    let dimensions = &Dimensions::new(4, 2);
    let cell = Cell::new(dimensions, 0, 0);                         //  1 |  2 |  4 |  8      1 | 2 | 3 | 4
		assert_eq!(cell.options, 255);                             		  // ------------------  =  --------------
     																															  // 16 | 32 | 64 | 128     5 | 6 | 7 | 8
		assert_eq!(cell.contains_option_at_position(0, 0), true);
		assert_eq!(cell.contains_option_at_position(1, 0), true);
		assert_eq!(cell.contains_option_at_position(2, 0), true);
		assert_eq!(cell.contains_option_at_position(3, 0), true);
		assert_eq!(cell.contains_option_at_position(0, 1), true);
		assert_eq!(cell.contains_option_at_position(1, 1), true);
		assert_eq!(cell.contains_option_at_position(2, 1), true);
		assert_eq!(cell.contains_option_at_position(3, 1), true);
		assert_eq!(cell.contains_option_at_position(4, 1), false);      // No bit set - overflow
		assert_eq!(cell.contains_option_at_position(0, 2), false);
  }
}

#[cfg(test)]
mod symbol {
  use crate::cell::{SYMBOLS, cell::Cell, dimensions::Dimensions, SetMethod};

  #[test]
  fn set_by_option_at_1() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0); 

    cell.set_by_option(1, SetMethod::User);
	  assert_eq!(cell.symbol(), '1');
  }

  #[test]
  fn set_by_option_at_2() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0); 

    cell.set_by_option(2, SetMethod::User);                         // 1 << 1 = 2
	  assert_eq!(cell.symbol(), '2');
  }

  #[test]
  fn set_by_option_at_a() {
    let dimensions = &Dimensions::new(4, 4);
    let mut cell = Cell::new(dimensions, 0, 0); 

    cell.set_by_option(1 << 9, SetMethod::User);
	  assert_eq!(cell.symbol(), 'A');
  }

  #[test]
  fn set_by_option_at_v() {
    let dimensions = &Dimensions::new(6, 6);
    let mut cell = Cell::new(dimensions, 0, 0); 

    cell.set_by_option(1 << 30, SetMethod::User);
	  assert_eq!(cell.symbol(), 'V');
  }

  #[test]
  fn set_by_option_at_0() {
    let dimensions = &Dimensions::new(6, 6);
    let mut cell = Cell::new(dimensions, 0, 0); 

    cell.set_by_option(1 << 35, SetMethod::User);
	  assert_eq!(cell.symbol(), '0');
  }

  #[test]
  fn set_by_position_4_5() {
    let dimensions = &Dimensions::new(6, 6);
    let mut cell = Cell::new(dimensions, 0, 0); 

    cell.set_by_position(4, 5, SetMethod::User);
	  assert_eq!(cell.symbol(), 'Z');
  }

  #[test]
  fn set_by_position_5_5() {                                        // Max symbol
    let dimensions = &Dimensions::new(6, 6);
    let mut cell = Cell::new(dimensions, 0, 0); 

    cell.set_by_position(5, 5, SetMethod::User);
	  assert_eq!(cell.symbol(), '0');
  }

  #[test]
  fn set_by_symbol_3_set_and_solved() {
    let dimensions = &Dimensions::new(2, 2);
    let mut cell = Cell::new(dimensions, 0, 0); 

    cell.set_by_symbol('3', SetMethod::User);
    assert_eq!(cell.symbol(), '3');
	  assert_eq!(cell.solved(), true);
	  assert_eq!(cell.set_method, SetMethod::User);
  }

  #[test]
  fn set_by_symbol_exhaustive() {
    let dimensions = &Dimensions::new(6, 6);
    let mut cell = Cell::new(dimensions, 0, 0); 

    SYMBOLS.iter().enumerate().for_each(|(i, x)| {
      cell.set_by_symbol(*x, SetMethod::User);
      assert_eq!(cell.symbol(), SYMBOLS[i]);
    });
  }
}

#[cfg(test)]
mod cell_3x3 {
  use crate::cell::{cell::Cell, dimensions::Dimensions, SetMethod};

  #[test]
  fn set_cell_unmodified() {
    let dimensions = &Dimensions::new(3, 3);
    let cell = Cell::new(dimensions, 0, 0);

    assert_eq!(cell.options, usize::pow(2, 9) - 1);                 // All options available i.e. 511
    assert_eq!(cell.total_options_remaining, dimensions.total);     // 3 * 3
    assert_eq!(cell.solved(), false);                               // Not solved
    assert_eq!(cell.set_method, SetMethod::Unset);

    assert_eq!(cell.contains_option_at_position(0, 0), true);       // Contains option at (0, 0)
    assert_eq!(cell.contains_option(0), false);                     // Does not contain option 0
    assert_eq!(cell.contains_option(1), true);                      // Contains option 1
    assert_eq!(cell.contains_option(3), true);                      // Contain either option 1 or 2
    assert_eq!(cell.contains_options(3), true);                     // Contain both options 1 and 2
  }

  #[test]
  fn set_at_position_0_2() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.set_by_position(0, 2, SetMethod::User);                  	// Set cell to column 0 row 2 i.e. symbol 7, bit 64
    assert_eq!(cell.total_options_remaining, 1);
    assert_eq!(cell.symbol(), '7');
    assert_eq!(cell.solved(), true);
    assert_eq!(cell.set_method, SetMethod::User);

    assert_eq!(cell.contains_option_at_position(0, 0), false);
    assert_eq!(cell.contains_option_at_position(0, 2), true);
    assert_eq!(cell.contains_option(0), false);
    assert_eq!(cell.contains_option(32), false);
    assert_eq!(cell.contains_option(64), true);
    assert_eq!(cell.contains_option(65), true);                     // bit 1 or 64
    assert_eq!(cell.contains_options(64), true);
    assert_eq!(cell.contains_options(65), false);                   // bit 1 and 64
  }

  #[test]
  fn set_option_4() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.set_by_option(4, SetMethod::User);                  	      // Set cell to options 4 i.e. highest of bits 1 and 4
    assert_eq!(cell.total_options_remaining, 1);
    assert_eq!(cell.symbol(), '3');
    assert_eq!(cell.solved(), true);                                // Only 1 bit set
    assert_eq!(cell.set_method, SetMethod::User);

    assert_eq!(cell.contains_option_at_position(0, 0), false);      // Only contains bit 4
    assert_eq!(cell.contains_option_at_position(2, 0), true);
    assert_eq!(cell.contains_option_at_position(0, 2), false);
    assert_eq!(cell.contains_option(0), false);
    assert_eq!(cell.contains_option(32), false);
    assert_eq!(cell.contains_option(1), false);
    assert_eq!(cell.contains_option(4), true);
    assert_eq!(cell.contains_option(5), true);                      // bit 1 or 4
    assert_eq!(cell.contains_option(7), true);                      // 1, 2 or 4
    assert_eq!(cell.contains_options(5), false);
    assert_eq!(cell.contains_options(4), true);
    assert_eq!(cell.contains_options(7), false);                    // bit 1, 2 and 4
  }

  #[test]
  fn reset_cell() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.set_by_position(0, 2, SetMethod::User);
    cell.reset();

    assert_eq!(cell.options, 511);
    assert_eq!(cell.total_options_remaining, 9);
    assert_eq!(cell.solved(), false);
    assert_eq!(cell.set_method, SetMethod::Unset);
  }

  #[test]
  fn remove_bit_16() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    assert_eq!(cell.remove_option_at_position(1, 1), false);        // Remove option bit = 16 - not last option
    assert_eq!(cell.total_options_remaining, 8);

  // 			expect(cell.json).toEqual({
	// 				rows:
	// 				[
	// 					{ columns: [{ symbol: '1' }, { symbol: '2' }, { symbol: '3' }] },
	// 					{ columns: [{ symbol: '4' }, { symbol: '5', strikeOut: true }, { symbol: '6' }] },
	// 					{ columns: [{ symbol: '7' }, { symbol: '8' }, { symbol: '9' }] }
	// 				]
	// 			});
  }

  #[test]
  fn bit_16_already_removed() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.remove_option_at_position(1, 1);                           // Continue from previous test

    assert_eq!(cell.remove_option(16), false);
    assert_eq!(cell.total_options_remaining, 8);
  }

  #[test]
  fn remove_bits_1_2_4() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.remove_option_at_position(1, 1);                           // Continue from previous tests

    assert_eq!(cell.remove_options(7), false);                      // Removed 4, 2 and 1
    assert_eq!(cell.total_options_remaining, 5);

  // 			expect(cell.json).toEqual({
	// 				rows:
	// 				[
	// 					{ columns: [{ symbol: '1', strikeOut: true }, { symbol: '2', strikeOut: true }, { symbol: '3', strikeOut: true }] },
	// 					{ columns: [{ symbol: '4' }, { symbol: '5', strikeOut: true }, { symbol: '6' }] },
	// 					{ columns: [{ symbol: '7' }, { symbol: '8' }, { symbol: '9' }] }
	// 				]
	// 			});
  }

  #[test]
  fn does_not_contain_bit_2() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.remove_option_at_position(1, 1);                           // Continue from previous tests
    cell.remove_options(7);

    assert_eq!(cell.options, 488);                                  // 488 = 000 101 111
    assert_eq!(cell.contains_option(2), false);
    assert_eq!(cell.remove_option_at_position(1, 0), false);        // 2 already removed
    assert_eq!(cell.total_options_remaining, 5);
  }
  
  #[test]
  fn options_removed_per_row() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.remove_option_at_position(1, 1);                           // Continue from previous tests
    cell.remove_options(7);

    let mut removed_options = Vec::with_capacity(dimensions.columns);

    cell.removed_options_per_row(0, &mut removed_options);
    assert_eq!(removed_options, [0, 1, 2]);                         // 0 0 0    - all removed from row 0
    cell.removed_options_per_row(1, &mut removed_options);
    assert_eq!(removed_options, [1]);                               // 1 0 1    - only 2nd option removed
    cell.removed_options_per_row(2, &mut removed_options);
    assert_eq!(removed_options, []);                                // 1 1 1    - no options removed
  }

  #[test]
  fn options_not_removed() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.remove_option_at_position(1, 1);                           // Continue from previous tests
    cell.remove_options(7);

    assert_eq!(cell.remove_options(488), false);                    // Attempt to remove all
    assert_eq!(cell.remove_options(511), false);
    assert_eq!(cell.total_options_remaining, 5);                    // Nothing removed
  }

  #[test]
  fn remove_bottom_row() {
    let dimensions = &Dimensions::new(3, 3);
    let mut cell = Cell::new(dimensions, 0, 0);

    cell.remove_option_at_position(1, 1);                           // Continue from previous tests
    cell.remove_options(7);

    assert_eq!(cell.remove_options(256 + 128 + 64), false);         // Remove bottom row
    // let mut removed_options = Vec::with_capacity(dimensions.columns);
    // cell.removed_options_per_row(2, &mut removed_options);
    // assert_eq!(removed_options, [0, 1, 2]);                         // All removed
  }
}

	// 		it("should have bottom row removed", () => {
	// 			expect(cell.removeOptions(256 + 128 + 64)).toBe(false);     // Remove bottom row
	// 			expect(cell.removedOptionsPerRow(2)).toEqual([0, 1, 2]);    // All removed
	// 			expect(cell.json).toEqual({
	// 				rows:
	// 				[
	// 					{ columns: [{ symbol: '1', strikeOut: true }, { symbol: '2', strikeOut: true }, { symbol: '3', strikeOut: true }] },
	// 					{ columns: [{ symbol: '4' }, { symbol: '5', strikeOut: true }, { symbol: '6' }] },
	// 					{ columns: [{ symbol: '7', strikeOut: true }, { symbol: '8', strikeOut: true }, { symbol: '9', strikeOut: true }] }
	// 				]
	// 			});
	// 			expect(cell.totalOptionsRemaining).toBe(2);
	// 			expect(cell.solved()).toBe(false);
	// 			expect(cell.setMethod).toBeNull();
	// 		});

	// 		it("should have bit 32 removed leaving bit 8", () => {
	// 			expect(cell.removeOptions(32 + 4 + 2 + 1)).toBe(true);  		// Only 32 removed leaving 8 - RETURNS last remaining
	// 			expect(cell.removedOptionsPerRow(1)).toEqual([1, 2]);      	// Only first bit in row left
	// 			expect(cell.json).toEqual({ symbol: '4' });

	// 			expect(cell.totalOptionsRemaining).toBe(1);
	// 			expect(cell.containsOptionAtPosition(0, 1)).toBe(true);
	// 			expect(cell.containsOption(8)).toBe(true);
	// 			expect(cell.symbol()).toBe("4");
	// 		});

	// 		it("should be solved", () => {
	// 			expect(cell.solved()).toBe(true);
	// 			expect(cell.setMethod).not.toBeNull();
	// 		});
	// 	});
	// });


// 	describe("Json", () => {
// 		let cell: ICell;

// 		beforeEach(() => {
// 			Cell.Constructor(2, 2);
// 			cell = new Cell(0, 0);
// 		});

// 		describe("Get", () => {
// 			it("should contain all symbols", () => {
// 				expect(cell.json).toEqual({ "rows": [{ "columns": [{ "symbol": "1" }, { "symbol": "2" }] }, { "columns": [{ "symbol": "3" }, { "symbol": "4" }] }] });
// 			});

// 			it("should have symbol 1 stuck out", () => {
// 				cell.removeOption(1);
// 				expect(cell.json).toEqual({ "rows": [{ "columns": [{ "symbol": "1", "strikeOut": true }, { "symbol": "2" }] }, { "columns": [{ "symbol": "3" }, { "symbol": "4" }] }] });
// 			});

// 			it("should be set directly", () => {
// 				cell.json.rows[0].columns[0].strikeOut = false;
// 				cell.json.rows[0].columns[1].strikeOut = true;
// 				expect(cell.json).toEqual({ "rows": [{ "columns": [{ "symbol": "1", "strikeOut": false }, { "symbol": "2", "strikeOut": true }] }, { "columns": [{ "symbol": "3" }, { "symbol": "4" }] }] });
// 			});

// 			it("should be set by reference", () => {
// 				const json: IJsonCell = cell.json;
// 				cell.removeOption(1);
// 				json.rows[0].columns[1].strikeOut = true;
// 				json.rows[1].columns[0].strikeOut = false;
// 				json.rows[1].columns[1].strikeOut = false;

// 				expect(cell.json).toEqual({ "rows": [{ "columns": [{ "symbol": "1", "strikeOut": true }, { "symbol": "2", "strikeOut": true }] }, { "columns": [{ "symbol": "3", "strikeOut": false }, { "symbol": "4", "strikeOut": false }] }] });
// 			});

// 			it("should be set to a symbol", () => {
// 				cell.setByOption(1, SetMethod.user);
// 				expect(cell.json).toEqual({ "symbol": "1", "setMethod": SetMethod.user });
// 			});

// 			it("should have options removed", () => {
// 				cell.removeOptions(3);
// 				expect(cell.json).toEqual({ "rows": [{ "columns": [{ "symbol": "1", "strikeOut": true }, { "symbol": "2", "strikeOut": true }] }, { "columns": [{ "symbol": "3" }, { "symbol": "4" }] }] });
// 			});

// 			it("should have options removed leaving symbol", () => {
// 				cell.removeOptions(7);
// 				expect(cell.json).toEqual({ "symbol": "4" });
// 			});
// 		});

// 		describe("Set", () => {
// 			it("should be set to default", () => {
// 				cell.setJson({ "rows": [{ "columns": [{ "symbol": "1", "strikeOut": false }, { "symbol": "2" }] }, { "columns": [{ "symbol": "3" }, { "symbol": "4", "strikeOut": false }] }] });
// 				expect(cell.json).toEqual({ "rows": [{ "columns": [{ "symbol": "1", "strikeOut": false }, { "symbol": "2" }] }, { "columns": [{ "symbol": "3" }, { "symbol": "4", "strikeOut": false }] }] });

// 				expect(cell.options).toBe(15);
// 				expect(cell.containsOptionAtPosition(0, 0)).toBe(true);
// 				expect(cell.containsOptionAtPosition(1, 0)).toBe(true);
// 				expect(cell.containsOptionAtPosition(0, 1)).toBe(true);
// 				expect(cell.containsOptionAtPosition(1, 1)).toBe(true);
// 				expect(cell.totalOptionsRemaining).toBe(4);
// 				expect(cell.setMethod).toBeNull();
// 			});

// 			it("should have struck out options", () => {
// 				cell.setJson({ "rows": [{ "columns": [{ "symbol": "1", "strikeOut": true }, { "symbol": "2", "strikeOut": false }] }, { "columns": [{ "symbol": "3" }, { "symbol": "4", "strikeOut": true }] }] });
// 				expect(cell.json).toEqual({ "rows": [{ "columns": [{ "symbol": "1", "strikeOut": true }, { "symbol": "2", "strikeOut": false }] }, { "columns": [{ "symbol": "3" }, { "symbol": "4", "strikeOut": true }] }] });

// 				expect(cell.options).toBe(15 - 1 - 8);
// 				expect(cell.containsOptionAtPosition(0, 0)).toBe(false);
// 				expect(cell.containsOptionAtPosition(1, 0)).toBe(true);
// 				expect(cell.containsOptionAtPosition(0, 1)).toBe(true);
// 				expect(cell.containsOptionAtPosition(1, 1)).toBe(false);
// 				expect(cell.totalOptionsRemaining).toBe(2);
// 				expect(cell.setMethod).toBeNull();
// 			});

// 			it("should be a symbol only", () => {
// 				cell.setJson({ "symbol": "1" });
// 				expect(cell.json).toEqual({ "symbol": "1" });
// 			});

// 			it("should be set by option to a symbol", () => {
// 				cell.setByOption(1, SetMethod.user);
// 				expect(cell.json).toEqual({ "symbol": "1", "setMethod": SetMethod.user });
// 			});

// 			it("should be set by position to a symbol", () => {
// 				cell.setByPosition(1, 1, SetMethod.user);
// 				expect(cell.json).toEqual({ "symbol": "4", "setMethod": SetMethod.user });
// 			});

// 			it("should be set by symbol to a symbol", () => {
// 				cell.setBySymbol("3", SetMethod.user);
// 				expect(cell.json).toEqual({ "symbol": "3", "setMethod": SetMethod.user });
// 			});

// 			it("should be fixed by option to a symbol", () => {
// 				cell.setByOption(1, SetMethod.loaded);
// 				expect(cell.json).toEqual({ "symbol": "1", "setMethod": SetMethod.loaded });
// 			});

// 			it("should be fixed by position to a symbol", () => {
// 				cell.setByPosition(1, 1, SetMethod.loaded);
// 				expect(cell.json).toEqual({ "symbol": "4", "setMethod": SetMethod.loaded });
// 			});

// 			it("should be fixed by symbol to a symbol", () => {
// 				cell.setBySymbol("3", SetMethod.loaded);
// 				expect(cell.json).toEqual({ "symbol": "3", "setMethod": SetMethod.loaded });
// 			});
// 		});

// 		describe("Deep copy", () => {
// 			let copy: ICell;

// 			it("should be equal as unmodified", () => {
// 				copy = new Cell(cell);
// 				expect(copy.json).toEqual(cell.json);
// 			});

// 			it("should not be equal as only one modified", () => {
// 				cell.json.rows[0].columns[0].symbol = 'x';
// 				expect(copy.json).not.toEqual(cell.json);
// 			});

// 			xit("should be equal as both modified", () => {
// 				copy = new Cell(cell);

// 				copy.json.rows[0].columns[0].symbol = 'x';
// 				expect(copy.json).toEqual(cell.json);
// 			});
// 		});
// 	});

// 	describe("3x3", () => {
// 		let cell: ICell;

// 		describe("Set", () => {
// 			beforeEach(() => {
// 				Cell.Constructor(3, 3);
// 				cell = new Cell(0, 0);
// 			});

// 			it("should be unmodified", () => {
// 				expect(cell.options).toBe(Math.pow(2, 3 * 3) - 1);          // All options available i.e. 511
// 				expect(cell.totalOptionsRemaining).toBe(3 * 3);
// 				expect(cell.solved()).toBe(false);                          // Not solved
// 				expect(cell.setMethod).toBeNull();

// 				expect(cell.containsOptionAtPosition(0, 0)).toBe(true);     // Contains option at (0, 0)
// 				expect(cell.containsOption(0)).toBe(false);                 // Does not contain option 0
// 				expect(cell.containsOption(1)).toBe(true);                  // Contains option 1
// 				expect(cell.containsOption(3)).toBe(true);                  // Contain either option 1 or 2
// 				expect(cell.containsOptions(3)).toBe(true);                 // Contain both options 1 and 2
// 			});

// 			it("should have cell (0, 2) set", () => {
// 				cell.setByPosition(0, 2, SetMethod.user);                  	// Set cell to column 0 row 2 i.e. symbol 7, bit 64
// 				expect(cell.totalOptionsRemaining).toBe(1);
// 				expect(cell.symbol()).toBe("7");
// 				expect(cell.solved()).toBe(true);
// 				expect(cell.setMethod).not.toBeNull();

// 				expect(cell.containsOptionAtPosition(0, 0)).toBe(false);
// 				expect(cell.containsOptionAtPosition(0, 2)).toBe(true);
// 				expect(cell.containsOption(0)).toBe(false);
// 				expect(cell.containsOption(32)).toBe(false);
// 				expect(cell.containsOption(64)).toBe(true);
// 				expect(cell.containsOption(65)).toBe(true);              		// bit 1 or 64
// 				expect(cell.containsOptions(64)).toBe(true);
// 				expect(cell.containsOptions(65)).toBe(false);               // bit 1 and 64
// 			});

// 			it("should have option 4 set", () => {
// 				cell.setByOption(4, SetMethod.user);                        // Set cell to options 4 i.e. highest of bits 1 and 4
// 				expect(cell.totalOptionsRemaining).toBe(1);
// 				expect(cell.symbol()).toBe("3");
// 				expect(cell.solved()).toBe(true);                           // Only 1 bit set
// 				expect(cell.setMethod).not.toBeNull();

// 				expect(cell.containsOptionAtPosition(0, 0)).toBe(false);    // Only contains bit 4
// 				expect(cell.containsOptionAtPosition(2, 0)).toBe(true);
// 				expect(cell.containsOptionAtPosition(0, 2)).toBe(false);
// 				expect(cell.containsOption(0)).toBe(false);
// 				expect(cell.containsOption(32)).toBe(false);
// 				expect(cell.containsOption(1)).toBe(false);
// 				expect(cell.containsOption(4)).toBe(true);
// 				expect(cell.containsOption(5)).toBe(true);                  // bit 1 or 4
// 				expect(cell.containsOption(7)).toBe(true);                  // 1, 2 or 4
// 				expect(cell.containsOptions(5)).toBe(false);
// 				expect(cell.containsOptions(4)).toBe(true);
// 				expect(cell.containsOptions(7)).toBe(false);                // bit 1, 2 and 4
// 			});

// 			it("should have options reset", () => {
// 				cell.reset();
// 				expect(cell.options).toBe(Math.pow(2, 3 * 3) - 1);          // All options reset i.e. 511
// 				expect(cell.totalOptionsRemaining).toBe(3 * 3);
// 				expect(cell.solved()).toBe(false);
// 				expect(cell.setMethod).toBeNull();
// 			});
// 		});

// 		describe("Options removed", () => {
// 			it("should have bit 16 removed", () => {
// 				Cell.Constructor(3, 3);
// 				cell = new Cell(0, 0);

// 				expect(cell.removeOptionAtPosition(1, 1)).toBe(false);   		// Remove option bit = 16 - not last option
// 				expect(cell.totalOptionsRemaining).toBe(8);
// 				expect(cell.json).toEqual({
// 					rows:
// 					[
// 						{ columns: [{ symbol: '1' }, { symbol: '2' }, { symbol: '3' }] },
// 						{ columns: [{ symbol: '4' }, { symbol: '5', strikeOut: true }, { symbol: '6' }] },
// 						{ columns: [{ symbol: '7' }, { symbol: '8' }, { symbol: '9' }] }
// 					]
// 				});
// 			});

// 			it("should have bit 16 already  removed", () => {
// 				expect(cell.removeOption(16)).toBe(false);            			// Already removed
// 				expect(cell.totalOptionsRemaining).toBe(8);
// 			});

// 			it("should have bits 1 + 2 + 4 removed", () => {
// 				expect(cell.removeOptions(7)).toBe(false);                	// Removed 4, 2 and 1
// 				expect(cell.totalOptionsRemaining).toBe(5);
// 				expect(cell.json).toEqual({
// 					rows:
// 					[
// 						{ columns: [{ symbol: '1', strikeOut: true }, { symbol: '2', strikeOut: true }, { symbol: '3', strikeOut: true }] },
// 						{ columns: [{ symbol: '4' }, { symbol: '5', strikeOut: true }, { symbol: '6' }] },
// 						{ columns: [{ symbol: '7' }, { symbol: '8' }, { symbol: '9' }] }
// 					]
// 				});
// 			});

// 			it("should not contain bit 2", () => {
// 				const options: number = cell.options;                				// 488 = 000 101 111
// 				expect(cell.containsOption(2)).toBe(false);
// 				expect(cell.removeOptionAtPosition(1, 0)).toBe(false);      // 2 already removed
// 				expect(cell.options).toBe(options);
// 				expect(cell.totalOptionsRemaining).toBe(5);
// 			});

// 			it("should have options removed", () => {
// 				expect(cell.removedOptionsPerRow(0)).toEqual([0, 1, 2]);    // 0 0 0    - all removed from row 0
// 				expect(cell.removedOptionsPerRow(1)).toEqual([1]);          // 1 0 1    - only 2nd option removed
// 				expect(cell.removedOptionsPerRow(2)).toEqual([]);           // 1 1 1    - no options removed
// 			});

// 			it("should have nothing removed", () => {
// 				expect(cell.removeOptions(488)).toBe(false);                // Attempt to remove all
// 				expect(cell.removeOptions(511)).toBe(false);
// 				expect(cell.totalOptionsRemaining).toBe(5);                 // Nothing removed
// 			});

// 			it("should have bottom row removed", () => {
// 				expect(cell.removeOptions(256 + 128 + 64)).toBe(false);     // Remove bottom row
// 				expect(cell.removedOptionsPerRow(2)).toEqual([0, 1, 2]);    // All removed
// 				expect(cell.json).toEqual({
// 					rows:
// 					[
// 						{ columns: [{ symbol: '1', strikeOut: true }, { symbol: '2', strikeOut: true }, { symbol: '3', strikeOut: true }] },
// 						{ columns: [{ symbol: '4' }, { symbol: '5', strikeOut: true }, { symbol: '6' }] },
// 						{ columns: [{ symbol: '7', strikeOut: true }, { symbol: '8', strikeOut: true }, { symbol: '9', strikeOut: true }] }
// 					]
// 				});
// 				expect(cell.totalOptionsRemaining).toBe(2);
// 				expect(cell.solved()).toBe(false);
// 				expect(cell.setMethod).toBeNull();
// 			});

// 			it("should have bit 32 removed leaving bit 8", () => {
// 				expect(cell.removeOptions(32 + 4 + 2 + 1)).toBe(true);  		// Only 32 removed leaving 8 - RETURNS last remaining
// 				expect(cell.removedOptionsPerRow(1)).toEqual([1, 2]);      	// Only first bit in row left
// 				expect(cell.json).toEqual({ symbol: '4' });

// 				expect(cell.totalOptionsRemaining).toBe(1);
// 				expect(cell.containsOptionAtPosition(0, 1)).toBe(true);
// 				expect(cell.containsOption(8)).toBe(true);
// 				expect(cell.symbol()).toBe("4");
// 			});

// 			it("should be solved", () => {
// 				expect(cell.solved()).toBe(true);
// 				expect(cell.setMethod).not.toBeNull();
// 			});
// 		});
// 	});
