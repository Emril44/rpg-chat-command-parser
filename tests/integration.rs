//! # Integration Tests for RPG Chat Command Parser
//!
//! These tests validate the integration of the parsing library with the application.

use rpg_chat_command_parser::parse_command;

/// Test parsing a valid command.
#[test]
fn test_integration_valid_command() {
    let input = "/cast fireball --power=high";
    let result = parse_command(input);
    assert!(result.is_ok());
}