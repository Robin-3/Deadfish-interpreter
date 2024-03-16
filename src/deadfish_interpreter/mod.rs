use command::Command;
use error::InterpreterError;
use interpreter::{Data, Interpreter};

mod command;
mod error;
mod interpreter;

const USAGE: &str = "./deadfish <df_code> \n\nDeadfish interpreter.\n\nArguments:\n  <df_code>        Deadfish code to be executed. Use only the following 4 instructions: idso";

// Function to interpret Deadfish code from command line arguments
pub fn deadfish_interpreter() -> Result<(String, Data), InterpreterError> {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        // Create a new Deadfish instance and execute the code
        let df = Interpreter::new();
        df.execute(Command::code_to_tokens(args[1].to_string())?)?;

        // Return the output as a tuple of String and Vec<u8>
        return Ok((df.get_output_as_string()?, df.get_output_as_vec()?));
    }

    // Return an error if the number of arguments is incorrect
    Err(InterpreterError::SintaxisError(USAGE.to_string()))
}
