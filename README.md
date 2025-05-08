# Brainfuck-rs
Brainfuck interpreter made with Rust

## Build
```plaintext
cargo build --release
```

## Usage
```plaintext
Usage: brainfuck-rs [option] ... [-c cmd | file]

Options:
-c cmd : program passed in as string (terminates option list)
-h     : print this help message and exit (also -? or --help)
-v     : verbose (trace tokens and operations)
-V     : print the Brainfuck-rs version number and exit (also --version)

Arguments:
file   : program read from script file
```
