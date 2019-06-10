mod bit_utils;
mod combinations;
use combinations::Combinations;

fn main() {
  let combinations = Combinations::new(9);                         // Columns * rows

  println!("Hello World! {}", bit_utils::number_of_bits_set(333));
  // println!("combinations {:?}", combinations.create_set_bits_lookup(1));

    let pick = 1;
    let from = [0, 1, 2, 3].to_vec();
    // let from = ["a", "b", "c", "d"];
		let items = combinations.select(&from, pick);

		// const expected: string[][] = [];
		// expected[0] = ["a"];
		// expected[1] = ["b"];
		// expected[2] = ["c"];
		// expected[3] = ["d"];

    print!("=============={:?}", items);

  // combinations.test();
}
