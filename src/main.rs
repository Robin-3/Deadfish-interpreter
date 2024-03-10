use deadfish_interpreter::deadfish_interpreter;

mod deadfish_interpreter;

// Main function to run the Deadfish interpreter
fn main() {
    // Match the result of the Deadfish interpreter function and print the output or error
    match deadfish_interpreter() {
        Ok(output) => println!("\"{}\" {:?}", output.0, output.1),
        Err(error) => eprintln!("{}", error),
    }
}
