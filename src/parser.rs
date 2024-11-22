use pest::Parser;
use std::collections::HashMap;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"] // Tells Pest where the grammar file is
struct CommandParser;

#[derive(Debug, PartialEq)]
pub struct ParsedCommand {
    pub verb: String,
    pub target: Option<String>,
    pub flags: HashMap<String, String>,
}

pub fn parse_command(input: &str) -> Result<ParsedCommand, String> {
    let mut parsed = CommandParser::parse(Rule::command, input)
        .map_err(|e| format!("Failed to parse command: {}", e))?
        .next()
        .ok_or("Failed to extract command rule.")?
        .into_inner();

    let mut verb = String::new();
    let mut target = None;
    let mut flags = HashMap::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::verb => verb = pair.as_str().to_string(),
            Rule::target => target = Some(pair.as_str().to_string()),
            Rule::flag => {
                let mut inner = pair.into_inner(); // Extract nested rules
                let key = inner.next().ok_or("Missing flag key.")?.as_str().to_string();
                let value = inner.next().ok_or("Missing flag value.")?.as_str().to_string();
                println!("Parsing flag: key = {:?}, value = {:?}", key, value);
                flags.insert(key, value);
            }
            _ => {}
        }
    }

    if verb.is_empty() {
        return Err("Missing verb in command.".to_string());
    }

    Ok(ParsedCommand { verb, target, flags })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_command() {
        let input = "/cast fireball --power=high";
        let result = parse_command(input).unwrap();
        assert_eq!(result.verb, "cast");
        assert_eq!(result.target, Some("fireball".to_string()));
        assert_eq!(result.flags.get("power"), Some(&"high".to_string()));
    }

    #[test]
    fn test_command_without_target() {
        let input = "/equip sword";
        let result = parse_command(input).unwrap();
        assert_eq!(result.verb, "equip");
        assert_eq!(result.target, Some("sword".to_string()));
        assert!(result.flags.is_empty());
    }
}
