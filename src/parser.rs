use pest::Parser;
use std::collections::HashMap;

use crate::CommandError;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"] // Tells Pest where the grammar file is
struct CommandParser;

#[derive(Debug, PartialEq)]
pub struct ParsedCommand {
    pub verb: String,
    pub target: Option<String>,
    pub flags: HashMap<String, String>,
}

pub fn parse_command(input: &str) -> Result<ParsedCommand, CommandError> {
    let mut parsed = CommandParser::parse(Rule::command, input)
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
