use std::env;
use std::io::Write;

use crate::{ast::ASTExecutable, dice_error::DiceError};

mod ast;
mod dice_error;
mod lexer;
mod parser;

fn main() {
    let mut verbose = false;
    if let Some(arg) = env::args().nth(1)
        && arg == "--v"
    {
        verbose = true;
    }

    print!("Please enter a dice algebra expression: ");
    if let Err(err) = std::io::stdout().flush() {
        eprintln!("Error: Failed to flush stdout buffer: {err}");
        std::process::exit(1);
    }

    let mut input_buffer = String::new();
    if let Err(err) = std::io::stdin().read_line(&mut input_buffer) {
        eprintln!("Error: Failed to read input: {err}");
        std::process::exit(1);
    };
    let input = input_buffer.trim_end();

    if let Err(err) = run(input, verbose) {
        eprintln!("Error! {}", err.message);
        std::process::exit(1);
    }
}

fn run(input: &str, verbose: bool) -> Result<(), DiceError> {
    let tokens = lexer::tokenize(input)?;

    let ast = parser::parse(&tokens)?;

    let result = ast.execute_ast(&mut rand::rng())?;

    if verbose {
        print!("{}", result.description);
    }
    println!("\nYour result is: {}", result.result);

    return Ok(());
}
