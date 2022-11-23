use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScholenError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Line unreachable")]
    ApplicationUnreachable,
}

pub type ScholenResult<T> = Result<T, ScholenError>;