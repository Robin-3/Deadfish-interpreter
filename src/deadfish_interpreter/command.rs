use super::error::InterpreterError;

pub type Commands = Vec<Command>;

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
    pub fn code_to_tokens(code: String) -> Result<Commands, InterpreterError> {
        let mut tokens: Commands = Vec::with_capacity(code.len());
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

    pub fn output_counter(commands: &Commands) -> usize {
        let mut counter = 0usize;
        for c in commands.iter() {
            if let Self::Output = c { counter += 1 }
        }
        
        counter
    }
}
