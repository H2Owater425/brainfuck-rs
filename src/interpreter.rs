use std::{error::Error, io::{stdin, stdout, Read, Stdin, Stdout, Write}};
use crate::parser::Operation;

pub struct Interpreter<'a> {
	operations: &'a [Operation],
	cells: [u8; 30000],
	pointer: usize,
	stdin: Stdin,
	stdout: Stdout,
	is_verbose: bool
}

impl<'a> Interpreter<'a> {
	pub fn new(operations: &'a [Operation], is_verbose: bool) -> Interpreter<'a> {
		Interpreter {
			operations: operations,
			cells: [0u8; 30000],
			pointer: 0,
			stdin: stdin(),
			stdout: stdout(),
			is_verbose: is_verbose
		}
	}

	pub fn move_pointer(self: &mut Self, value: i16) -> Result<(), Box<dyn Error>> {
		let new_pointer = self.pointer as i16 + value;

		if new_pointer < 0 || new_pointer >= 30000 {
			return Err("Pointer must be between 0 to 29,999".into());
		}

		self.pointer = new_pointer as usize;

		Ok(())
	}

	pub fn increase_value(self: &mut Self, value: u8) -> () {
		self.cells[self.pointer] = ((self.cells[self.pointer] as u16 + value as u16) % 256) as u8;
	}

	pub fn output(self: &mut Self) -> Result<(), Box<dyn Error>> {
		self.stdout.write(&[self.cells[self.pointer]])?;
		self.stdout.flush()?;

		Ok(())
	}

	pub fn input(self: &mut Self) -> Result<(), Box<dyn Error>> {
		self.stdin.read_exact(&mut self.cells[self.pointer..self.pointer + 1])?;

		Ok(())
	}

	pub fn jump(self: &mut Self, is_zero: bool, operations: &'a [Operation]) -> Result<(), Box<dyn Error>> {
		if is_zero {
			while self.cells[self.pointer] != 0 {
				self.execute(Some(operations))?;
			}
		} else {
			while self.cells[self.pointer] == 0 {
				self.execute(Some(operations))?;
			}
		}

		Ok(())
	}

	pub fn execute(self: &mut Self, operations: Option<&'a [Operation]>) -> Result<(), Box<dyn Error>> {
		let operations: &'a [Operation] = operations.unwrap_or(self.operations);

		for operation in operations {
			if self.is_verbose {
				println!("{:#?}", operation);
			}

			match operation {
				Operation::MovePointer(value) => self.move_pointer(*value)?,
				Operation::IncreaseValue(value) => self.increase_value(*value),
				Operation::Output => self.output()?,
				Operation::Input => self.input()?,
				Operation::Jump(is_zero, operations) => self.jump(*is_zero, operations)?,
			}
		}

		Ok(())
	}
}