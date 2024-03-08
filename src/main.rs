use thiserror::Error;

// Define custom error types using the `thiserror` crate
#[derive(Error, Debug)]
enum DeadfishError {
    #[error("Usage: {0}")]
    SintaxisError(String),
    #[error("Character instruction unknown: '{0}'")]
    InstruccionUnknown(char),
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
    tokens: Vec<Command>,
}

impl Deadfish {
    // Constructor to create a new Deadfish interpreter instance
    fn new(code: &str) -> Result<Deadfish, DeadfishError> {
        // Generate tokens from the Deadfish code
        let tokens = Deadfish::tokens(code.into())?;

        // Return a new Deadfish instance
        Ok(Deadfish {
            value: 0,
            output: Vec::new(),
            tokens,
        })
    }

    // Execute the Deadfish code
    fn execute(&mut self) {
        for token in self.tokens.iter() {
            let mut val = self.value as u16;

            // Match each command and perform the corresponding operation
            match token {
                Command::Increase => val += 1,
                Command::Decrease => val += u8::MAX as u16,
                Command::Square => val *= val,
                Command::Output => self.output.push(self.value),
            }
            self.value = val as u8
        }
    }

    // Generate tokens from Deadfish code
    fn tokens(code: String) -> Result<Vec<Command>, DeadfishError> {
        let mut tokens: Vec<Command> = Vec::new();
        for c in code.chars() {
            // Match each character to its corresponding Deadfish command
            match c {
                'i' => tokens.push(Command::Increase),
                'd' => tokens.push(Command::Decrease),
                's' => tokens.push(Command::Square),
                'o' => tokens.push(Command::Output),
                char => return Err(DeadfishError::InstruccionUnknown(char)),
            }
        }

        // Return the generated tokens
        Ok(tokens)
    }
}

// Function to interpret Deadfish code from command line arguments
fn deadfish_interpreter() -> Result<(String, Vec<u8>), DeadfishError> {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        // Create a new Deadfish instance and execute the code
        let mut df = Deadfish::new(&args[1])?;
        df.execute();

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
