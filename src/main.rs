mod bit_utils;
mod combinations;
use combinations::Combinations;

fn main() {
  let combinations = Combinations::new(4);                         // Columns * rows

  println!("Hello World! {}", bit_utils::number_of_bits_set(333));
  // println!("combinations {:?}", combinations.create_set_bits_lookup(1));

    let from = ['a', 'b', 'c', 'd'].to_vec();
		let pick = 4;
		let items = combinations.select(from, pick);

		// const expected: string[][] = [];
		// expected[0] = ["a"];
		// expected[1] = ["b"];
		// expected[2] = ["c"];
		// expected[3] = ["d"];

    print!("=============={:?}", items);

  // combinations.test();
}
