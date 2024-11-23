use clap::{Arg, Command};

fn main() {
    let matches = Command::new("rpg-chat-command-parser")
        .version("0.1.0")
        .author("Maksym Khomenko")
        .about("Parses RPG-style chat commands")
        .arg(
            Arg::new("command")
                .help("Command to parse")
                .required(true)
                .index(1), // Specifies position (first argument)
        )
        .get_matches();

    let command = matches.get_one::<String>("command").expect("Command is required");

    match parse_command(command) {
        Ok(parsed) => println!("Parsed command: {:?}", parsed),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn parse_command(command: &str) -> Result<String, String> {
    // Placeholder function for now
    Ok(format!("Command received: {}", command))
}