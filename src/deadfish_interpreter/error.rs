use thiserror::Error;

// Define custom error types using the `thiserror` crate
#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Usage: {0}")]
    SintaxisError(String),
    #[error("Character instruction unknown: '{0}'")]
    InstruccionUnknown(char),
    #[error("Execution Error: Tokens not loaded")]
    TokensUnknown,
    #[error("Execution Error: Tokens cannot be overwritten")]
    TokensOverwritten,
    #[error("Execution Error: No code was executed")]
    OutputUnknown,
    #[error("Unexpected Error: Modifying output value")]
    OutputOverwritten,
}
