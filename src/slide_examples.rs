// Any of these examples can be run using the following command with cargo:
// cargo test [testname] -- --nocapture
// 
// The -- --nocapture parameter avoids the capturing of the standard out
// printing that happens in some of these. For example, in module_2::print_string(),
// there is a print! macro statement that would not print without the use of
// the -- --nocapture parameter. 
//
// If you don't care about standard out, then these are all easily runnable 
// using simply:
// cargo test
//
// But doing that will only give you useful information if you're also pairing
// that with the use of a debugging tool.


pub mod module_2
{
	pub fn print_string()
	{
		let hello_string: &str = "hello";
		print!("{}\n", hello_string);
	}

	// Debug this on the line where the `character` 
	// variable is defined and watch these numbers
	// magically get turned into characters!
	pub fn print_string_bytes()
	{
		let hello_string: [u8; 5] = [104u8, 101u8, 108u8, 108u8, 111u8];

		// character_code begins life as a u8... an 8-bit unsigned integer
		// https://doc.rust-lang.org/std/primitive.u8.html
		for character_code in hello_string
		{
			let character: char = character_code as char;
			print!("{}", character);
		}

		print!("\n");
	}

	// When debugging, arrays and Vecs are not the same,
	// and inspecting the contents of a Vec is quite different
	// when considering the implementation at a deep level, 
	// within memory.
	pub fn memory_addresses_in_vec()
	{
		let item_one: String = String::from("hello");
		let item_two: String = String::from("world");
		let item_three: String = String::from("one");
		let item_four: String = String::from("two");
		
		// The array is allocated on the stack and has a much
		// simpler structure in memory than that of a vec.
		let item_array: [String; 4] = [item_one, item_two, item_three, item_four];

		// Copying the array into a vec allocates it to the heap
		// and has a more complex structure
		let string_vec: Vec<String> = Vec::from(item_array.clone());

		// The length of both of these may appear similar on the surface
		print!("item_array.len(): {}\n", item_array.len());
		print!("string_vec.len(): {}\n", string_vec.len());
	}
}

pub mod module_3
{
	use crate::orders::simulate_an_order;
	use crate::structs::Context;

	pub fn check_outstanding_orders()
	{
		let context: &mut Context = &mut simulate_an_order();
		let orders = &context.orders;

		print!("CURRENT ORDERS\n");
		for order in orders
		{
			context.print_order(order);
		}
	}
}

pub mod module_4
{
	use crate::orders::calculate_orders_parallel;

	pub fn calc_order_totals()
	{
		calculate_orders_parallel();
	}
}

#[cfg(test)]
pub mod module_2_test
{
	use crate::slide_examples::module_2::*;

	#[test]
	pub fn test_print_string()
	{
		print_string();
	}

	#[test]
	pub fn test_print_string_bytes()
	{
		print_string_bytes();
	}

	#[test]
	fn test_memory_addresses_in_vec()
	{
		memory_addresses_in_vec();
	}
}

#[cfg(test)]
pub mod module_3_test
{
	use crate::slide_examples::module_3::*;

	#[test]
	fn test_check_outstanding_orders()
	{
		check_outstanding_orders();
	}
}