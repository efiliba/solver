use crate::utils::bit_utils::{highest_bit_position};
// use crate::utils::bit_utils::{power_of_2_bit_positions, highest_bit_position, number_of_bits_set};
use crate::cell::{SetMethod, dimensions::Dimensions, json_cell::JsonCell};

pub struct Cell<'a> {
  dimensions: &'a Dimensions,

  pub column: usize,
  pub row: usize,
  options: usize,
  pub json: JsonCell,
  total_options_remaining: usize,

  set_method: SetMethod,
  set_column: usize,
  set_row: usize
}

impl<'a> Cell<'a> {
  pub fn new(dimensions: &'a Dimensions, column: usize, row: usize) -> Self {
    Cell {
      dimensions,
      column,
      row,
      options: (1 << dimensions.total) - 1,                         // Set all bits
      json: JsonCell::new(dimensions),
      total_options_remaining: dimensions.total,
      set_method: SetMethod::Unset,
      set_column: 0,
      set_row: 0
    }
  }

  pub fn reset(&mut self) {
    self.total_options_remaining = self.dimensions.total;
    self.options = (1 << self.dimensions.total) - 1;       // Set all bits

      // self.json = JsonCell::new(COLUMNS, ROWS);
    //   this.json = { rows: [] };                                  			// Set JSON representation to all options available 
		// for (let row: number = 0, index: number = 0; row < Cell.rows; row++) {
		// 	const columns: IJsonCellColumn[] = [];
		// 	for (let column = 0; column < Cell.columns; column++) {
		// 		columns.push({ symbol: Cell.symbols[index++] });
		// 	}
		// 	this.json.rows.push({ columns: columns });
		// }
  }

	pub fn solved(&self) -> bool {
		self.total_options_remaining == 1
  }
  
  pub fn remove_option_at_position(&mut self, column: usize, row: usize) -> bool { // Return if last option left after removing this option
		let mut last_option_found = false;
    let bit = 1 << self.dimensions.columns * row + column;
    
    if self.options & bit > 0 {                                	  // Check if option to remove exists
      self.options &= !bit;
      self.total_options_remaining -= 1;
      if self.total_options_remaining == 1 {
        self.set_remaining_option(self.options);                  // Set last remaining option's column and row 
        // 		this.json = { symbol: Cell.symbols[powerOf2BitPositions[this.options]] };
        self.set_method = SetMethod::Calculated;
        last_option_found = true;
        // 	} else {
          // 		this.json.rows[row].columns[column].strikeOut = true; 			// Only set strikeOut to true if option removed - else leave empty   
      }
    }
    
		last_option_found
  }
  
  pub fn test(&self) {
    println!("Call from cell: {} {}: {}: {}", self.dimensions.columns, self.dimensions.rows, self.total_options_remaining, self.options);
  }

  fn set_remaining_option(&mut self, options: usize) {
    let index = highest_bit_position(options);
    self.set_column = index % self.dimensions.columns;
    self.set_row = index / self.dimensions.columns >> 0;
  }
  
// 	constructor(public column: any, public row?: number) {
// 		if (typeof column === "number") {
// 			this.reset();
// 		} else {                                                     		// Copy constructor
// 			const copy: Cell = column;
// 			this.column = copy.column;
// 			this.row = copy.row;
// 			this.setMethod = copy.setMethod;

// 			this.options = copy.options;
// 			this.setColumn = copy.setColumn;
// 			this.setRow = copy.setRow;
// 			this.totalOptionsRemaining = copy.totalOptionsRemaining;

// 			this.json = copy.json;
			
// 			if (copy.json.rows) {
// 				this.json = { rows: [] };
// 				for (let row: number = 0; row < copy.json.rows.length; row++) {
// 					this.json.rows[row] = { columns: [] };
// 					const jsonColumns: IJsonCellColumn[] = copy.json.rows[row].columns;
// 					for (let column = 0; column < jsonColumns.length; column++) {
// 						this.json.rows[row].columns[column] = { symbol: jsonColumns[column].symbol };
// 						if (jsonColumns[column].strikeOut) {
// 							this.json.rows[row].columns[column].strikeOut = jsonColumns[column].strikeOut;
// 						}
// 					}
// 				}
// 			} else {
// 				this.json = { symbol: copy.json.symbol, setMethod: copy.json.setMethod };
// 			} 
// 		}
// 	}



// export interface IJsonCellColumn {
// 	symbol?: string;
// 	strikeOut?: boolean;
// 	highlight?: boolean;
// }

// export interface IJsonCellRow {
// 	columns: IJsonCellColumn[];
// }

// export interface IJsonCell {
// 	rows?: IJsonCellRow[];
// 	symbol?: string;
// 	setMethod?: SetMethod;
// }

// export interface ICell {
// 	setMethod: SetMethod;
// 	options: number;
// 	json: IJsonCell;
// 	totalOptionsRemaining: number;

// 	equal(cell: ICell): boolean;

// 	symbol(): string;
// 	getColumn(): number;
// 	getRow(): number;
// 	solved(): boolean;
// 	removeOption(option: number): boolean;
// 	removeOptionAtPosition(column: number, row: number): boolean;
// 	toggleRemoveOptionAtPositionShallow(column: number, row: number): void;
// 	toggleHighlightOptionAtPosition(column: number, row: number): void;
// 	removeOptions(remove: number): boolean;
// 	setByPosition(column: number, row: number, setMethod: SetMethod): void;
// 	setByOption(option: number, setMethod: SetMethod): void;
// 	setBySymbol(symbol: string, setMethod: SetMethod): void;
// 	reset(): void;
// 	containsOption(option: number): boolean;
// 	containsOptionAtPosition(column: number, row: number): boolean;
// 	containsOptions(checkOptions: number): boolean;
// 	containsSymbol(symbol: string): boolean;
// 	removedOptionsPerRow(row: number): number[];
// 	setJson(json: IJsonCell): void;
// }

// export class Cell implements ICell {
// 	private static symbols: string;
// 	private static columns: number;
// 	private static rows: number;

// 	public setMethod: SetMethod;
// 	public options: number;
// 	public setColumn: number;
// 	public setRow: number;
// 	public json: IJsonCell;

// 	public totalOptionsRemaining: number;

// 	static Constructor(columns: number, rows: number) {               // Static constructor
// 		Cell.symbols = "123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0"
// 		Cell.columns = columns;
// 		Cell.rows = rows;
// 	}

// 	// constructor(column: number, row: number);
// 	// constructor(copy: ICell);
// 	constructor(public column: any, public row?: number) {
// 		if (typeof column === "number") {
// 			this.reset();
// 		} else {                                                     		// Copy constructor
// 			const copy: Cell = column;
// 			this.column = copy.column;
// 			this.row = copy.row;
// 			this.setMethod = copy.setMethod;

// 			this.options = copy.options;
// 			this.setColumn = copy.setColumn;
// 			this.setRow = copy.setRow;
// 			this.totalOptionsRemaining = copy.totalOptionsRemaining;

// 			this.json = copy.json;
			
// 			if (copy.json.rows) {
// 				this.json = { rows: [] };
// 				for (let row: number = 0; row < copy.json.rows.length; row++) {
// 					this.json.rows[row] = { columns: [] };
// 					const jsonColumns: IJsonCellColumn[] = copy.json.rows[row].columns;
// 					for (let column = 0; column < jsonColumns.length; column++) {
// 						this.json.rows[row].columns[column] = { symbol: jsonColumns[column].symbol };
// 						if (jsonColumns[column].strikeOut) {
// 							this.json.rows[row].columns[column].strikeOut = jsonColumns[column].strikeOut;
// 						}
// 					}
// 				}
// 			} else {
// 				this.json = { symbol: copy.json.symbol, setMethod: copy.json.setMethod };
// 			} 
// 		}
// 	}

// 	public reset() {
// 		this.setMethod = null;

// 		this.options = (1 << Cell.columns * Cell.rows) - 1;            	// Set all bits
// 		this.setColumn = -1;
// 		this.setRow = -1;
// 		this.totalOptionsRemaining = Cell.columns * Cell.rows;

// 		this.json = { rows: [] };                                  			// Set JSON representation to all options available 
// 		for (let row: number = 0, index: number = 0; row < Cell.rows; row++) {
// 			const columns: IJsonCellColumn[] = [];
// 			for (let column = 0; column < Cell.columns; column++) {
// 				columns.push({ symbol: Cell.symbols[index++] });
// 			}
// 			this.json.rows.push({ columns: columns });
// 		}
// 	}

// 	public equal(cell: Cell): boolean {
// 		return (this.setColumn === cell.setColumn || cell.setColumn === -1) &&
// 			(this.setRow === cell.setRow || cell.setRow === -1) && this.options === cell.options;
// 		//return this.options === cell.options;
// 	}

// 	public symbol(): string {
// 		return Cell.symbols[this.setRow * Cell.columns + this.setColumn];
// 	}

// 	public getColumn(): number {
// 		return this.column;
// 	}

// 	public getRow(): number {
// 		return this.row;
// 	}

// 	public solved(): boolean {
// 		return this.totalOptionsRemaining === 1;
// 	}

// 	public removeOptionAtPosition(column: number, row: number): boolean {	// Return if last option left after removing this option
// 		let lastOptionFound: boolean = false;

// 		const bit: number = 1 << Cell.columns * row + column;
// 		if (this.options & bit) {                                				// Check if option to remove exists
// 			this.options &= ~bit;
// 			if (--this.totalOptionsRemaining === 1) {
// 				this.setRemainingOption(this.options);                     	// Set last remaining option's column and row 
// 				this.json = { symbol: Cell.symbols[powerOf2BitPositions[this.options]] };
// 				this.setMethod = SetMethod.calculated;
// 				lastOptionFound = true;
// 			} else {
// 				this.json.rows[row].columns[column].strikeOut = true; 			// Only set strikeOut to true if option removed - else leave empty   
// 			}
// 		}

// 		return lastOptionFound;
// 	}

// 	public toggleRemoveOptionAtPositionShallow(column: number, row: number): void {
// 		const cell = this.json.rows[row].columns[column];
// 		cell.strikeOut = !cell.strikeOut;
// 	}

// 	public toggleHighlightOptionAtPosition(column: number, row: number): void {
// 		const cell = this.json.rows[row].columns[column];
// 		cell.highlight = !cell.highlight;
// 	}

// 	public removeOption(option: number): boolean {                   	// Return if last option left after removing this option
// 		let lastOptionFound: boolean = false;

// 		if (this.options & option && this.totalOptionsRemaining > 1) { 	// Check if option to remove exists and not last option
// 			this.options &= ~option;
// 			if (--this.totalOptionsRemaining === 1) {
// 				this.setRemainingOption(this.options);                     	// Set last remaining option's column and row 
// 				this.json = { symbol: Cell.symbols[powerOf2BitPositions[this.options]] };
// 				this.setMethod = SetMethod.calculated;
// 				lastOptionFound = true;
// 			}
// 			else {
// 				const index: number = powerOf2BitPositions[option];
// 				this.json.rows[index / Cell.columns >> 0].columns[index % Cell.columns].strikeOut = true;
// 			}
// 		}

// 		return lastOptionFound;
// 	}

// 	public removeOptions(remove: number): boolean {
// 		let lastOptionFound: boolean = false;

// 		let removeOptions: number = this.options & remove;
// 		if (removeOptions && this.totalOptionsRemaining > 1 && this.options & ~remove) {	// Remove options iff cell contains other options
// 			this.options -= removeOptions;
// 			this.totalOptionsRemaining -= numberOfBitsSet(removeOptions);

// 			if (this.totalOptionsRemaining === 1) {
// 				this.setRemainingOption(this.options);                			// Set last remaining option's column and row 
// 				this.json = { symbol: Cell.symbols[powerOf2BitPositions[this.options]] };
// 				this.setMethod = SetMethod.calculated;
// 				lastOptionFound = true;
// 			}
// 			else
// 				while (removeOptions) {
// 					let highestBitPos: number = highestBitPosition(removeOptions);
// 					this.json.rows[highestBitPos / Cell.columns >> 0].columns[highestBitPos % Cell.columns].strikeOut = true;
// 					removeOptions -= 1 << highestBitPos;
// 				}
// 		}

// 		return lastOptionFound;
// 	}

// 	public setByPosition(column: number, row: number, setMethod: SetMethod) {
// 		this.clearAllExceptAtPosition(this.setColumn = column, this.setRow = row, this.setMethod = setMethod);
// 	}

// 	private setByIndex(index: number, setMethod: SetMethod) {
// 		this.clearAllExceptAtPosition(this.setColumn = index % Cell.columns, this.setRow = index / Cell.columns >> 0, this.setMethod = setMethod);
// 	}

// 	public setByOption(option: number, setMethod: SetMethod) {
// 		this.setByIndex(powerOf2BitPositions[option], setMethod);
// 	}

// 	public setBySymbol(symbol: string, setMethod: SetMethod){
// 		this.setByIndex(Cell.symbols.indexOf(symbol), setMethod);
// 	}

// 	public containsOption(option: number): boolean {
// 		return (this.options & option) > 0;
// 	}

	pub fn contains_option_at_position(&self, column: usize, row: usize) -> bool {
    let bit = 1 << row * self.dimensions.columns + column;
    (self.options & bit) > 0
	}

// 	public containsOptions(checkOptions: number): boolean {
// 		return (this.options & checkOptions) === checkOptions;
// 	}

// 	public containsSymbol(symbol: string): boolean {
// 		const index: number = Cell.symbols.indexOf(symbol);
// 		return (this.options & 1 << index) > 0; // eslint-disable-line no-mixed-operators
// 	}

// 	private setRemainingOption(options: number) {
// 		const index: number = highestBitPosition(options);
// 		this.setColumn = index % Cell.columns;
// 		this.setRow = index / Cell.columns >> 0;
// 	}

// 	// private clearAllExcept(option: number, fix: boolean) {
// 	// 	this.options = option;
// 	// 	this.json = { symbol: Cell.symbols[powerOf2BitPositions[option]], fixed: fix };
// 	// 	this.totalOptionsRemaining = 1;
// 	// }

// 	private clearAllExceptAtPosition(column: number, row: number, setMethod: SetMethod) {
// 		this.options = 1 << Cell.columns * row + column;
// 		this.json = { symbol: Cell.symbols[powerOf2BitPositions[this.options]], setMethod };
// 		this.totalOptionsRemaining = 1;
// 	}

// 	public removedOptionsPerRow(row: number): number[] {
// 		const removedOptions: number[] = [];
// 		for (let column: number = 0, bit: number = 1 << row * Cell.columns; column < Cell.columns; column++, bit <<= 1) {	// bit = 1 << row * columns + column
// 			if (!(this.options & bit)) {
// 				removedOptions.push(column);
// 			}
// 		}

// 		return removedOptions;
// 	}

// 	// Remove options iff cell contains other options
// 	public removeIfExtraOptions(options: number): boolean {
// 		return this.totalOptionsRemaining > 1 && (this.options & ~options) > 0 && this.removeOptions(options);
// 	}

// 	public setJson(json: IJsonCell) {
// 		if (json.rows) {
// 			this.options = 0;
// 			this.totalOptionsRemaining = 0;
// 			for (let row: number = 0, option: number = 1; row < json.rows.length; row++) {
// 				const columns: IJsonCellColumn[] = json.rows[row].columns;
// 				for (let column = 0; column < columns.length; column++, option <<= 1) {
// 					if (!columns[column].strikeOut) {
// 						this.options += option;
// 						this.totalOptionsRemaining++;
// 					}
// 				}
// 			}
// 		} else {
// 			this.setBySymbol(json.symbol, json.setMethod);
// 		}

// 		this.json = json;
// 	}
// }
}
