use std::env::
{
	current_exe as current_executable_directory,
	args as command_line_arguments
};

mod orders;
mod slide_examples;
mod structs;

fn print_help_text()
{
	print!("\
debugging-rust

MODULE 2
========
module_2:print_string
module_2:print_string_bytes
module_2:memory_addresses_in_vec

MODULE 3
========
module_3:check_outstanding_orders

MODULE 4
========
module_4:calc_order_totals
")
}

fn handle_arguments(_program_context: &mut structs::Context)
{
	let mut argument_index: i32 = -1i32;
	for argument in command_line_arguments()
	{
		argument_index += 1;
		
		// The first argument is always the executable name and
		// should therefore be ignored
		if argument_index == 0i32 { continue; }

		// Any other argument is an attempt to call course code that
		// is held within these example modules, in which case we want
		// to route it based on what we're attempting to call off the 
		// program's help text labels.
		match &argument[..]
		{
			"--help" => print_help_text(),
			"module_2:print_string" => slide_examples::module_2::print_string(),
			"module_2:print_string_bytes" => slide_examples::module_2::print_string_bytes(),
			"module_2:memory_addresses_in_vec" => slide_examples::module_2::memory_addresses_in_vec(),
			"module_3:check_outstanding_orders" => slide_examples::module_3::check_outstanding_orders(),
			"module_4:calc_order_totals" => slide_examples::module_4::calc_order_totals(),
			_ => print!("Argument: {} not recognized.\n", argument)
		}
	}
}

fn main() 
{
	let program_context: &mut structs::Context = &mut structs::Context::new();
	handle_arguments(program_context);
}
