use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command syntax")]
    InvalidSyntax,
    #[error("Missing key in flag")]
    MissingFlagKey,
    #[error("Missing value in flag")]
    MissingFlagValue,
    #[error("Missing verb in command")]
    MissingVerb,
}
