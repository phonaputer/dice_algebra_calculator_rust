use std::io::Write;

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

    println!("You wrote: {input}!");
}
