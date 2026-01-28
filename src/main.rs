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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn run__big_ol_roll_with_all_the_fixings__returns_correct_result() {
        let input = "d5 + 2d6h1 - 3d100l2 + (10 - 2 * 2) / 2";
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let tokens = lexer::tokenize(input).unwrap();
        let tree = parser::parse(&tokens).unwrap();
        let result = tree.execute_ast(&mut rng).unwrap();

        assert_eq!(-33, result.result);
        assert_eq!(
            concat!(
                "\nRolling d5...\nYou rolled: 4\n",
                "\nRolling 2d6...\nYou rolled: 4\nYou rolled: 3\n",
                "\nRolling 3d100...\nYou rolled: 18\nYou rolled: 26\nYou rolled: 96\n"
            ),
            result.description
        );
    }
}
