use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScholenError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Line unreachable: {0}")]
    ApplicationUnreachable(String),
    #[error("Invalid html")]
    ParseError
}

pub type ScholenResult<T> = Result<T, ScholenError>;