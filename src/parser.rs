//! # Command Parser
//!
//! This module defines the core functionality for parsing RPG-style chat commands. It uses the
//! [Pest](https://docs.rs/pest) parser library for grammar-based parsing.
//!
//! ## Grammar
//! The grammar is defined in the grammar.pest file and supports the following:
//! - **Verbs**: The primary action of the command (e.g., /cast, /attack).
//! - **Targets**: Optional targets for the action (e.g., /cast fireball).
//! - **Flags**: Key-value pairs for additional parameters (e.g., --power=high).
//!
//! ## Example
//! use rpg_chat_command_parser::parse_command;
//!
//! let input = "/cast fireball --power=high";
//! let parsed = parse_command(input).unwrap();
//! assert_eq!(parsed.verb, "cast");
//! assert_eq!(parsed.target, Some("fireball".to_string()));
//! assert_eq!(parsed.flags.get("power"), Some(&"high".to_string()));
//!
use pest::Parser;
use std::collections::HashMap;

use crate::CommandError;
/// Pest parser definition for command parsing.
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"] // Tells Pest where the grammar file is
struct CommandParser;
/// Represents a parsed command with its components.
#[derive(Debug, PartialEq)]
pub struct ParsedCommand {
    /// The main verb of the command (e.g., "cast").
    pub verb: String,
    /// The optional target of the command (e.g., "fireball").
    pub target: Option<String>,
    /// A map of flag key-value pairs (e.g., --power=high).
    pub flags: HashMap<String, String>,
}
/// Parses an input string into a ParsedCommand.
///
/// # Errors
/// Returns a CommandError if the input does not match the grammar.
///
/// # Example
/// let input = "/cast fireball --power=high";
/// let parsed = parse_command(input).unwrap();
/// assert_eq!(parsed.verb, "cast");
/// assert_eq!(parsed.target, Some("fireball".to_string()));
/// assert_eq!(parsed.flags.get("power"), Some(&"high".to_string()));

pub fn parse_command(input: &str) -> Result<ParsedCommand, CommandError> {
    let parsed = CommandParser::parse(Rule::command, input)
        .map_err(|e| {
            println!("Debug: Parsing failed with error: {:?}", e);
            CommandError::InvalidSyntax
        })?
        .next()
        .ok_or_else(|| {
            println!("Debug: No top-level rule matched");
            CommandError::InvalidSyntax
        })?
        .into_inner();

    let mut verb = String::new();
    let mut target = None;
    let mut flags = HashMap::new();

    for pair in parsed {
        println!(
            "Debug: Top-level Pair = {:?}, Rule = {:?}",
            pair.as_str(),
            pair.as_rule()
        );
        match pair.as_rule() {
            Rule::verb => verb = pair.as_str().to_string(),
            Rule::target => target = Some(pair.as_str().to_string()),
            Rule::flag => {
                println!(
                    "Debug: Pair = {:?}, Rule = {:?}",
                    pair.as_str(),
                    pair.as_rule()
                );
                let mut inner = pair.into_inner(); // Extract nested key and value rules

                let key = inner
                    .next()
                    .ok_or(CommandError::MissingFlagKey)? // Missing key
                    .as_str()
                    .to_string();
                println!("Debug: Parsed key = {:?}", key);

                // Explicitly validate the key
                if key.is_empty() || key.starts_with("=") || key.contains("--") {
                    println!("Debug: Invalid key detected = {:?}", key);
                    return Err(CommandError::MissingFlagKey);
                }

                let value_pair = inner.next().ok_or(CommandError::MissingFlagValue)?; // Missing value
                let value = value_pair.as_str().to_string();

                if value.is_empty() {
                    return Err(CommandError::MissingFlagValue);
                }

                println!("Debug: Parsed value = {:?}", value);
                println!("Debug: Parsed key = {:?}, value = {:?}", key, value);
                flags.insert(key, value);
            }
            Rule::bad_flag => {
                println!("Debug: Malformed flag detected = {:?}", pair.as_str());
                return Err(CommandError::InvalidSyntax);
            }
            _ => {}
        }
    }

    if verb.is_empty() {
        return Err(CommandError::MissingVerb);
    }

    Ok(ParsedCommand {
        verb,
        target,
        flags,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_verb() {
        let input = "--power=high"; // No verb at the start
        let result = parse_command(input);
        assert!(matches!(result, Err(CommandError::InvalidSyntax))); // Updated to match syntax error
    }

    #[test]
    fn test_missing_flag_key() {
        let input = "/cast fireball --=high"; // Missing key
        let result = parse_command(input);
        assert!(matches!(result, Err(CommandError::MissingFlagKey)));
    }

    #[test]
    fn test_invalid_flag_key() {
        let input = "/cast fireball --==high"; // Missing key
        let result = parse_command(input);
        assert!(matches!(result, Err(CommandError::MissingFlagKey)));
    }

    #[test]
    fn test_missing_flag_value() {
        let input = "/cast fireball --power="; // Missing value
        let result = parse_command(input);
        assert!(matches!(result, Err(CommandError::MissingFlagValue)));
    }

    #[test]
    fn test_valid_command() {
        let input = "/cast fireball --power=high";
        let result = parse_command(input).unwrap();
        assert_eq!(result.verb, "cast");
        assert_eq!(result.target, Some("fireball".to_string()));
        assert_eq!(result.flags.get("power"), Some(&"high".to_string()));
    }
}