use super::error::InterpreterError;

// Enum to represent the Deadfish language commands
#[derive(Clone)]
pub enum Command {
    Increase,
    Decrease,
    Square,
    Output,
}

impl Command {
    // Generate tokens from Deadfish code
    pub fn code_to_tokens(code: String) -> Result<Vec<Self>, InterpreterError> {
        let mut tokens: Vec<Command> = Vec::new();
        for c in code.chars().rev() {
            // Match each character to its corresponding Deadfish command
            match c {
                'i' => tokens.push(Self::Increase),
                'd' => tokens.push(Self::Decrease),
                's' => tokens.push(Self::Square),
                'o' => tokens.push(Self::Output),
                char => return Err(InterpreterError::InstruccionUnknown(char)),
            }
        }

        // Return the generated tokens
        Ok(tokens)
    }
}
