use std::io::{Read, Seek, Write};

pub trait ReadWrite: Read + Write + Seek {}

impl<T: Read + Write + Seek> ReadWrite for T {}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const HELP_MESSAGE: &str = r#"Usage: brainfuck-rs [option] ... [-c cmd | file]

Options:
-c cmd : program passed in as string (terminates option list)
-h     : print this help message and exit (also -? or --help)
-v     : verbose (trace tokens and operations)
-V     : print the Brainfuck-rs version number and exit (also --version)

Arguments:
file   : program read from script file"#;