use std::io::Error;

use crate::common::ReadWrite;

#[derive(Debug, PartialEq)]
pub enum Token {
	Left,
	Right,
	Increase,
	Decrease,
	Output,
	Input,
	JumpIfNotZero,
	JumpIfZero
}

pub struct Lexer<T: ReadWrite> {
	reader: T
}

impl<T: ReadWrite> Lexer<T> {
	pub fn new(reader: T) -> Result<Lexer<T>, Error> {
		Ok(Lexer {
			reader: reader
		})
	}

	pub fn tokenize(self: &mut Self) -> Result<Vec<Token>, Error> {
		let mut tokens: Vec<Token> = Vec::new();
		let mut buffer: [u8; 1] = [0];

		while self.reader.read(&mut buffer)? != 0 {
			match buffer[0] {
				b'<' => tokens.push(Token::Left),
				b'>' => tokens.push(Token::Right),
				b'+' => tokens.push(Token::Increase),
				b'-' => tokens.push(Token::Decrease),
				b'.' => tokens.push(Token::Output),
				b',' => tokens.push(Token::Input),
				b'[' => tokens.push(Token::JumpIfZero),
				b']' => tokens.push(Token::JumpIfNotZero),
				_ => ()
			}
		}

		Ok(tokens)
	}
}