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
    #[error("Execution Error: {0} cannot be overwritten")]
    DataOverwritten(String),
    #[error("Execution Error: No code was executed")]
    OutputUnknown,
}
