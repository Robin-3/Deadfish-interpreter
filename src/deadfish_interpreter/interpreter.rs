use super::{command::Command, error::InterpreterError};
use std::cell::OnceCell;

// Struct to represent the Deadfish interpreter
#[derive(Default)]
pub struct Interpreter {
    output: OnceCell<Vec<u8>>,
    tokens: OnceCell<Vec<Command>>,
}

impl Interpreter {
    // Constructor to create a new Deadfish interpreter instance
    pub fn new() -> Self {
        Self::default()
    }

    fn set_tokens(&self, tokens: Vec<Command>) -> Result<(), InterpreterError> {
        self.tokens
            .set(tokens)
            .map_err(|_| InterpreterError::TokensOverwritten)?;

        Ok(())
    }

    // Execute the Deadfish code
    fn run_code(&self) -> Result<(), InterpreterError> {
        if self.output.get().is_some() {
            return Ok(());
        }

        match self.tokens.get() {
            Some(tokens) => {
                let mut value = 0u8;
                let mut output: Vec<u8> = Vec::new();
                let mut tokens = tokens.clone();

                while let Some(token) = tokens.pop() {
                    // Match each command and perform the corresponding operation
                    match token {
                        Command::Increase => value = value.wrapping_add(1),
                        Command::Decrease => value = value.wrapping_sub(1),
                        Command::Square => value = value.wrapping_pow(2),
                        Command::Output => output.push(value),
                    }
                }

                self.output
                    .set(output)
                    .map_err(|_| InterpreterError::OutputOverwritten)?;
            }
            None => return Err(InterpreterError::TokensUnknown),
        }

        Ok(())
    }

    pub fn execute(&self, tokens: Vec<Command>) -> Result<(), InterpreterError> {
        self.set_tokens(tokens)?;
        self.run_code()
    }

    pub fn get_output_as_vec(&self) -> Result<Vec<u8>, InterpreterError> {
        match self.output.get() {
            Some(output) => Ok(output.clone()),
            None => Err(InterpreterError::OutputUnknown),
        }
    }

    pub fn get_output_as_string(&self) -> Result<String, InterpreterError> {
        match self.output.get() {
            Some(output) => Ok(String::from_utf8_lossy(output.as_slice()).to_string()),
            None => Err(InterpreterError::OutputUnknown),
        }
    }
}
