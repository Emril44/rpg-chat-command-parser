use rpg_chat_command_parser::parse_command;

#[test]
fn test_integration_valid_command() {
    let input = "/cast fireball --power=high";
    let result = parse_command(input);
    assert!(result.is_ok());
}
