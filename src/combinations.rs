pub struct Combinations {
  set_bits_lookup_table: Vec<Vec<usize>>
}

impl Combinations {
  pub fn new(max_items_select_from: usize) -> Combinations {
    let set_bits = create_set_bits_lookup(max_items_select_from);
    let mut set_bits_lookup_table: Vec<Vec<usize>> = Vec::with_capacity(max_items_select_from);

    for index in 0..=max_items_select_from {
      set_bits_lookup_table.push(Vec::new());

      for bit in 0..set_bits.len() {                                // Get indices of items with respective choices
        if set_bits[bit] == index {
					set_bits_lookup_table[index].push(bit);
				}
      }
    }

    Combinations { set_bits_lookup_table }
  }

  pub fn select(&self, from: Vec<char>, pick: usize) -> Vec<Vec<char>> {
		// Get bit flags used to select the combinations from the lookup table, up to the number of items to select from
		let set_bits = 1 << from.len();
		let lookup_table = &self.set_bits_lookup_table[pick];
		let mut combinations: Vec<Vec<char>> = Vec::with_capacity(lookup_table.len());

    for index in 0..lookup_table.len() {
      combinations.push(Vec::new());
			if lookup_table[index] < set_bits {
				combinations[index].extend_from_slice(&self.select_elements(&from, lookup_table[index]));
			}
		}

		combinations
	}

	// Return elements where the index is in the select bit flag
	fn select_elements(&self, from: &Vec<char>, select: usize) -> Vec<char> {
		//SelectElementsDelegate<T> selectElements = (elements, select) => { return elements.Where((x, i) => (1 << i & select) > 0); };
		let mut elements = Vec::with_capacity(from.len());
    for index in 0..from.len() {
			if (1 << index) & select > 0 {
				elements.push(from[index]);
			}
		}

		elements
	}
}

// Populate array with the number of bits set i.e. [0] => 0, [1] => 1, [2] => 1, [3] => 2, ..., [333] => 5 (i.e. 101001101 has 5 bits set)
fn create_set_bits_lookup(n: usize) -> Vec<usize> {
  fn next_values(x: usize) -> [usize; 4] {
    [x, x + 1, x + 1, x + 2]
  }

  let mut lookup_table: Vec<usize> = Vec::new();
  lookup_table.extend_from_slice(&next_values(0));                  // Starting values { 0, 1, 1, 2 }
  let mut table_size = 4;
  for _ in 2..n {
    let offset = table_size >> 2;
    for j in 0..(table_size >> 1) - offset {    
      lookup_table.extend_from_slice(&next_values(lookup_table[j + offset]));
    }
    table_size <<= 1;
  }

  lookup_table
}
