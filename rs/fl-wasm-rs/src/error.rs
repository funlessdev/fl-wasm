use thiserror::Error;

/// An error that can occur in a FunLess function.
#[derive(Error, Debug)]
pub enum FLError {
    #[error("error while executing function: {0}")]
    ExecError(String),

    #[error("function not implemented")]
    NotImplemented,
}
