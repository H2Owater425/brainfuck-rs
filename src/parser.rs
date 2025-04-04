use std::error::Error;

use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
	MovePointer(i16),
	IncreaseValue(u8),
	Output,
	Input,
	Jump(bool/* is_zero */, Vec<Operation>),
}

pub struct Parser {
	tokens: Vec<Token>,
	length: usize,
	index: usize
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Parser {
		let length: usize = tokens.len();

		Parser {
			tokens: tokens,
			length: length,
			index: 0
		}
	}

	pub fn parse(self: &mut Self, stopper: Option<Token>) -> Result<Vec<Operation>, Box<dyn Error>> {
		let mut operations: Vec<Operation> = Vec::new();

		while self.index < self.length {
			match self.tokens[self.index] {
				Token::Left => {
					if let Some(Operation::MovePointer(value)) = operations.last_mut() {
						if *value == -29999 {
							return Err("Left instruction must not be executed continuously 29,999 times".into());
						}
						
						*value -= 1;
					} else {
						operations.push(Operation::MovePointer(-1));
					}
				},
				Token::Right => {
					if let Some(Operation::MovePointer(value)) = operations.last_mut() {
						if *value == 29999 {
							return Err("Right instruction must not be executed continuously 29,999 times".into());
						}
						
						*value += 1;
					} else {
						operations.push(Operation::MovePointer(1));
					}
				},
				Token::Decrease => {
					if let Some(Operation::IncreaseValue(value)) = operations.last_mut() {
						*value = ((*value as u16 + 255) % 256) as u8;
					} else {
						operations.push(Operation::IncreaseValue(255));
					}
				},
				Token::Increase => {
					if let Some(Operation::IncreaseValue(value)) = operations.last_mut() {
						*value = ((*value as u16 + 1) % 256) as u8;
					} else {
						operations.push(Operation::IncreaseValue(1));
					}
				},
				Token::Input => operations.push(Operation::Input),
				Token::Output => operations.push(Operation::Output),
				Token::JumpIfZero => {
					if stopper == Some(Token::JumpIfZero) {
						break;
					}
					
					self.index += 1;

					operations.push(Operation::Jump(true /* is_zero */, self.parse(Some(Token::JumpIfNotZero))?));
				},
				Token::JumpIfNotZero => {
					if stopper == Some(Token::JumpIfNotZero) {
						break;
					}

					self.index += 1;

					operations.push(Operation::Jump(false /* is_zero */, self.parse(Some(Token::JumpIfZero))?));
				}
			}

			self.index += 1;
		}

		// TODO: More optimization tweaks

		// Token::JumpIfNotZero is start of code
		if let Some(Operation::Jump(is_zero, inner_operations)) = operations.first() {
			if stopper.is_none() && inner_operations.is_empty() {
				return Ok(if *is_zero {
					operations[1..].to_vec()
				} else {
					Vec::new()
				});
			}
		}

		let mut optimized_operations: Vec<Operation> = Vec::new();

		for operation in operations {
			match operation {
				Operation::MovePointer(0) | Operation::IncreaseValue(0) => continue,
				operation => optimized_operations.push(operation),
			}
		}

		Ok(optimized_operations)
	}
}