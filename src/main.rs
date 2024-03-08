use thiserror::Error;

// Define custom error types using the `thiserror` crate
#[derive(Error, Debug)]
enum DeadfishError {
    #[error("Usage: {0}")]
    SintaxisError(String),
    #[error("Character instruction unknown: '{0}'")]
    InstruccionUnknown(char),
    #[error("Execution Error: Tokens not loaded")]
    TokensUnknown,
}

// Enum to represent the Deadfish language commands
enum Command {
    Increase,
    Decrease,
    Square,
    Output,
}

// Struct to represent the Deadfish interpreter
struct Deadfish {
    value: u8,
    output: Vec<u8>,
    tokens: Option<Vec<Command>>,
}

impl Deadfish {
    // Constructor to create a new Deadfish interpreter instance
    fn new() -> Self {
        // Return a new Deadfish instance
        Deadfish {
            value: 0,
            output: Vec::new(),
            tokens: None,
        }
    }

    // Execute the Deadfish code
    fn execute(&mut self) -> Result<&mut Self, DeadfishError> {
        match self.tokens.as_mut() {
            Some(tokens) => {
                while let Some(token) = tokens.pop() {
                    // Match each command and perform the corresponding operation
                    match token {
                        Command::Increase => self.value = self.value.wrapping_add(1),
                        Command::Decrease => self.value = self.value.wrapping_sub(1),
                        Command::Square => self.value = self.value.wrapping_pow(2),
                        Command::Output => self.output.push(self.value),
                    }
                }
            }
            None => return Err(DeadfishError::TokensUnknown),
        }

        Ok(self)
    }

    // Generate tokens from Deadfish code
    fn tokens(&mut self, code: String) -> Result<&mut Self, DeadfishError> {
        let mut tokens: Vec<Command> = Vec::new();
        for c in code.chars().rev() {
            // Match each character to its corresponding Deadfish command
            match c {
                'i' => tokens.push(Command::Increase),
                'd' => tokens.push(Command::Decrease),
                's' => tokens.push(Command::Square),
                'o' => tokens.push(Command::Output),
                char => return Err(DeadfishError::InstruccionUnknown(char)),
            }
        }

        // Save the generated tokens
        self.tokens = Some(tokens);
        Ok(self)
    }
}

// Function to interpret Deadfish code from command line arguments
fn deadfish_interpreter() -> Result<(String, Vec<u8>), DeadfishError> {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        // Create a new Deadfish instance and execute the code
        let mut df = Deadfish::new();
        df.tokens(args[1].clone())?.execute()?;

        let output = String::from_utf8_lossy(df.output.as_slice()).to_string();
        // Return the output as a tuple of String and Vec<u8>
        return Ok((output, df.output));
    }

    // Return an error if the number of arguments is incorrect
    Err(DeadfishError::SintaxisError("./deadfich <df_code> \n\nDeadfish interpreter.\n\nArguments:\n  <df_code>        Deadfish code to be executed. Use only the following 4 instructions: idso".to_string()))
}

// Main function to run the Deadfish interpreter
fn main() {
    // Match the result of the Deadfish interpreter function and print the output or error
    match deadfish_interpreter() {
        Ok(output) => println!("\"{}\" {:?}", output.0, output.1),
        Err(error) => eprintln!("{}", error),
    }
}
