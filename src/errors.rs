//! # Error Handling for RPG Chat Command Parser
//!
//! This module defines custom errors for the command parser, using the [`thiserror`](https://docs.rs/thiserror) crate.
//!
//! ## Errors
//! - **`InvalidSyntax`**: Indicates a syntax error in the input command.
//! - **`MissingFlagKey`**: A flag is missing its key.
//! - **`MissingFlagValue`**: A flag is missing its value.
//! - **`MissingVerb`**: The command is missing a verb.

use thiserror::Error;

/// Custom error type for command parsing errors.
#[derive(Error, Debug)]
pub enum CommandError {
    /// The command syntax is invalid.
    #[error("Invalid command syntax")]
    InvalidSyntax,

    /// A flag is missing its key.
    #[error("Missing key in flag")]
    MissingFlagKey,

    /// A flag is missing its value.
    #[error("Missing value in flag")]
    MissingFlagValue,

    /// The command is missing a verb.
    #[error("Missing verb in command")]
    MissingVerb,
}