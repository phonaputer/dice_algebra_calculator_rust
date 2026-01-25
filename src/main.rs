use std::io::Write;

mod lexer;

fn main() {
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

    match lexer::tokenize(input) {
        Ok(tokens) => {
            println!("\nFound the following tokens...");
            for token in tokens {
                println!("Token: {:?}, Integer: {}", token.token_type, token.integer);
            }
        }
        Err(err) => {
            eprintln!("Error! {}", err.message);
            std::process::exit(1);
        }
    };
}
