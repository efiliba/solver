// Population count
pub fn number_of_bits_set(bits: usize) -> usize {
  let mut count = 0;
  let mut remaining_bits = bits;
  while remaining_bits > 0 {
    remaining_bits &= remaining_bits - 1;
    count += 1;
  }

  count
}

// Return bits set within all passed elements (not XOR)
pub fn bitwise_or(elements: &[usize]) -> usize {
  let mut total_ored = 0;
  for element in elements.iter() {
    total_ored |= element;;
  }

  total_ored
}

// XOR all the values passed in to find an only option
pub fn only_option(options: &[usize]) -> (bool, usize) {
  let mut option = 0;
  let mut filled = 0;
  for element in options.iter() {
    if element & element - 1 > 0 {                                  // Not a single base of 2 number (1, 2, 4, 8, ...)
      filled |= option & element;
      option ^= element;                                            // XOR
    }
  }

  option &= !filled;
  (option > 0 && (option & option - 1) == 0, option)                // Single base of 2 number, but not 0
}

// Index of first item in array containing bit
pub fn containing_bit_index(array: &[usize], bit: usize) -> usize {
  let mut index = 0;
  while index < array.len() && (array[index] & bit) == 0 {
    index += 1;
  }

  index
}

pub fn highest_bit_position(v: usize) -> usize {
  let multiply_de_bruijn_bit_position = [0, 9, 1, 10, 13, 21, 2, 29, 11, 14, 16, 18, 22, 25, 3, 30, 8, 12, 20, 28, 15, 17, 24, 7, 19, 27, 23, 6, 26, 5, 4, 31,
    0, 9, 1, 10, 13, 21, 2, 29, 11, 14, 16, 18, 22, 25, 3, 30, 8, 12, 20, 28, 15, 17, 24, 7, 19, 27, 23, 6, 26, 5, 4, 31];  // Duplicated

  let mut u = v;
  u |= u >> 1;                                                      // first round down to one less than a power of 2 
  u |= u >> 2;
  u |= u >> 4;
  u |= u >> 8;
  u |= u >> 16;

  multiply_de_bruijn_bit_position[(u * 0x07C4ACDD >> 27) + 32]
}

pub fn power_of_2_bit_positions(bit: usize) -> usize {
  let mut remaining_bit = bit;
  let mut index = 0;
  while remaining_bit > 1 {
    remaining_bit >>= 1;
    index += 1;
  }

  index
}

// export const power_of_2_bit_positions: IHashMapOfPowerOf2 = {
//     1: 0, 2: 1, 4: 2, 8: 3, 16: 4, 32: 5, 64: 6, 128: 7, 256: 8, 512: 9, 1024: 10, 2048: 11, 4096: 12, 8192: 13, 16384: 14, 32768: 15, 65536: 16, 131072: 17, 262144: 18,
//     524288: 19, 1048576: 20, 2097152: 21, 4194304: 22, 8388608: 23, 16777216: 24, 33554432: 25, 67108864: 26, 134217728: 27, 268435456: 28, 536870912: 29, 1073741824: 30, 2147483648: 31
// }

#[cfg(test)]
mod number_of_bits_set {
  use super::number_of_bits_set;

  #[test]
  fn it_returns_number_of_bits_set() {
    assert_eq!(number_of_bits_set(333), 5);	                        // Population count i.e. 333 = 101001101 i.e. 5 bits set
  }
}

#[cfg(test)]
mod set_bits {
  use super::bitwise_or;

  #[test]
  fn it_should_have_all_bits_set() {
    let elements = [1, 2, 4, 8];                    	              // 0001 | 0010 | 0100 | 1000 = 1111
    assert_eq!(bitwise_or(&elements), 15);	                        // Population count i.e. 333 = 101001101 i.e. 5 bits set
  }

  #[test]
  fn it_should_have_duplicate_bits_set_only_once() {
    let elements = [1, 2, 3];                    	                  // 01 | 10 | 11  = 11
    assert_eq!(bitwise_or(&elements), 3);
  }

  #[test]
  fn it_should_only_have_bits_set_if_any_item_contains_that_bit() {
    let elements = [2, 6, 12];                                      // 0010 | 0110 | 1100 = 1110
    assert_eq!(bitwise_or(&elements), 14);
  }
}

#[cfg(test)]
mod only_option {
  use super::only_option;

  #[test]
  fn it_should_not_have_any_bits_set() {
    let xor_bits = [1, 2, 3];                   				            // 01 ^ 10 ^ 11  = 00
    assert_eq!(only_option(&xor_bits), (false, 3));
  }

  #[test]
  fn it_should_have_all_bits_set() {
    let xor_bits = [1, 2, 4, 8];                   				          // 0001 ^ 0010 ^ 0100 ^ 1000 = 1111
    assert_eq!(only_option(&xor_bits), (false, 0));                 // All bits set i.e. singulare bit required
  }

  #[test]
  fn it_should_have_option_found_at_bit_2() {
    let xor_bits = [5, 6, 9, 12];                     	            // 0101 ^ 0110 ^ 1001 ^ 1100 = 0010
    assert_eq!(only_option(&xor_bits), (true, 2));
  }

  #[test]
  fn it_should_not_have_a_singular_option_set() {
    let xor_bits = [3, 6, 12];                      		            // 0011 ^ 0110 ^ 1100 = 1001
    assert_eq!(only_option(&xor_bits), (false, 9));
  }

  #[test]
  fn it_should_only_have_bit_8_set() {
    let xor_bits = [3, 7, 12];                       	              // 0011 ^ 0111 ^ 1100 = 1000
    assert_eq!(only_option(&xor_bits), (true, 8));
  }
}

#[cfg(test)]
mod first_index_of_item_in_array_containing_bit {
  use super::containing_bit_index;

  #[test]
  fn it_should_have_bit_1_set_at_index_2() {
    let array = [0, 2, 3, 4];                                 			// 000, 010, 011, 100 
    assert_eq!(containing_bit_index(&array, 1), 2);                 // Index of first item that has bit 1 set - only item 3 has bit 1 set
  }

  #[test]
  fn it_should_have_bit_2_set_at_index_1() {
    let array = [0, 2, 3, 4];
    assert_eq!(containing_bit_index(&array, 2), 1);              	  // Index of first item that has bit 2 set
  }

  #[test]
  fn it_should_have_bit_4_set_at_index_3() {
    let array = [0, 2, 3, 4];
    assert_eq!(containing_bit_index(&array, 4), 3);             	  // Index of first item that has bit 4 set
  }

  #[test]
  fn it_should_have_index_out_of_range() {
    let array = [0, 2, 3, 4];
	  assert_eq!(containing_bit_index(&array, 8), array.len());  	    // Bit 8 not set => index out of range
  }

  #[test]
  fn it_should_not_have_bit_0_found_ie_out_of_range() {
    let array = [0, 2, 3, 4];
    assert_eq!(containing_bit_index(&array, 0), array.len());   	  // Bit 0 not found => index out of range
  }
}

#[cfg(test)]
mod highest_bit_position {
  use super::highest_bit_position;

  #[test]
  fn it_should_not_exist() {
    assert_eq!(highest_bit_position(0), 0);
  }

  #[test]
  fn it_should_be_0_in_1() {
    assert_eq!(highest_bit_position(1), 0);
  }

  #[test]
  fn it_should_be_1_in_10() {
    assert_eq!(highest_bit_position(2), 1);
  }

  #[test]
  fn it_should_be_1_in_11() {
    assert_eq!(highest_bit_position(3), 1);
  }

  #[test]
  fn it_should_be_2_in_100() {
    assert_eq!(highest_bit_position(4), 2);
  }

  #[test]
  fn it_should_be_2_in_101() {
    assert_eq!(highest_bit_position(5), 2);
  }

  #[test]
  fn it_should_be_2_in_110() {
    assert_eq!(highest_bit_position(6), 2);
  }

  #[test]
  fn it_should_be_2_in_111() {
    assert_eq!(highest_bit_position(7), 2);
  }

  #[test]
  fn it_should_be_3_in_1000() {
    assert_eq!(highest_bit_position(8), 3);
  }

  #[test]
  fn it_should_be_3_in_1001() {
    assert_eq!(highest_bit_position(9), 3);
  }

  #[test]
  fn it_should_be_4_in_10000() {
    assert_eq!(highest_bit_position(16), 4);
  }

  #[test]
  fn it_should_be_4_in_10001() {
    assert_eq!(highest_bit_position(17), 4);
  }

  #[test]
  fn it_should_be_4_in_10010() {
    assert_eq!(highest_bit_position(18), 4);
  }

  #[test]
  fn it_should_match_highest_bit_position_function() {
    fn local_highest_bit_position(value: usize) -> usize {
      let mut index = 0;
      let mut bit = 1;
      while bit <= value {
        bit <<= 1;
        index += 1;
      }

      index - 1
    }

    for index in 1..32 {
      assert_eq!(highest_bit_position(index), local_highest_bit_position(index));
    }
	}
}

#[cfg(test)]
mod power_of_2_bit_position {
  use super::power_of_2_bit_positions;

  #[test]
  fn should_match() {
    assert_eq!(power_of_2_bit_positions(1), 0);
    assert_eq!(power_of_2_bit_positions(2), 1);
    assert_eq!(power_of_2_bit_positions(4), 2);
    assert_eq!(power_of_2_bit_positions(8), 3);
    assert_eq!(power_of_2_bit_positions(16), 4);
    assert_eq!(power_of_2_bit_positions(32), 5);
    assert_eq!(power_of_2_bit_positions(64), 6);
    assert_eq!(power_of_2_bit_positions(128), 7);
    assert_eq!(power_of_2_bit_positions(256), 8);
    assert_eq!(power_of_2_bit_positions(512), 9);
    assert_eq!(power_of_2_bit_positions(1024), 10);

    for index in 0..32 {
      assert_eq!(power_of_2_bit_positions(1 << index), index);
    }

    assert_eq!(power_of_2_bit_positions(2147483648), 31);
  }
}
