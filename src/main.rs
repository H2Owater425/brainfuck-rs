use std::{env::{args, Args}, error::Error, fs::File, io::Cursor, iter::Skip, process::exit};
use interpreter::Interpreter;
use common::{ReadWrite, HELP_MESSAGE, VERSION};
use lexer::{Lexer, Token};
use parser::Parser;

mod lexer;
mod parser;
mod interpreter;
mod common;

struct Configuration {
	reader: Option<Box<dyn ReadWrite>>,
	is_file: bool,
	is_verbose: bool
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut arguments: Skip<Args> = args().skip(1);
	let mut configuration: Configuration = Configuration {
		reader: None,
		is_file: true,
		is_verbose: false
	};

	while let Some(argument) = arguments.next() {
		if configuration.reader.is_none() {
			if argument.starts_with("-") {
				match argument.as_str() {
					"-c" => {
						if let Some(code) = arguments.next() {
							configuration.reader = Some(Box::new(Cursor::new(code.bytes().collect::<Vec<u8>>())));
							configuration.is_file = true;
		
							continue;
						}
		
						return Err("Argument must exist after -c option".into());
					},
					"-h" | "-?" => {
						println!("{}", HELP_MESSAGE);

						return Ok(());
					},
					"-V" => {
						println!("Brainfuck-rs {}", VERSION);

						return Ok(());
					},
					"-v" => {
						configuration.is_verbose = true;
		
						continue;
					},
					_ => {
						return Err("Arugment must be valid".into());
					}
				}
			}

			configuration.reader = Some(Box::new(File::open(argument)?));

			continue;
		}

		break;
	}

	let mut lexer: Lexer<Box<dyn ReadWrite>> = Lexer::new(configuration.reader.unwrap_or_else(|| {
		println!("{}", HELP_MESSAGE);
		
		exit(1);
	}))?;

	let tokens: Vec<Token> = lexer.tokenize()?;
	
	if configuration.is_verbose {
		println!("{:#?}", tokens);
	}

	let mut parser: Parser = Parser::new(tokens);
	
	let operations: Vec<parser::Operation>= parser.parse(None)?;

	if configuration.is_verbose {
		println!("{:#?}", operations);
	}

	let mut interpreter: Interpreter = Interpreter::new(&operations, configuration.is_verbose);

	interpreter.execute(None)?;

	Ok(())
}
