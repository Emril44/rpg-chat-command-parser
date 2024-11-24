use clap::{Arg, Command};
use rpg_chat_command_parser::parse_command;
use std::fs;

/// Main entry point for the RPG Chat Command Parser binary.
fn main() {
    // CLI argument parsing
    let matches = Command::new("rpg-chat-command-parser")
        .version("0.1.0")
        .author("Maksym Khomenko")
        .about("Parses RPG-style chat commands")
        .subcommand(
            Command::new("parse")
                .about("Parse a command string")
                .arg(Arg::new("command").help("Command to parse").required(true)),
        )
        .subcommand(
            Command::new("file")
                .about("Parse commands from a file")
                .arg(
                    Arg::new("file")
                        .help("File containing commands to parse")
                        .required(true),
                ),
        )
        .subcommand(Command::new("info").about("Display additional help information"))
        .get_matches();

    // Match the subcommand and execute the corresponding logic
    match matches.subcommand() {
        Some(("parse", sub_m)) => {
            let command = sub_m.get_one::<String>("command").unwrap();
            match parse_command(command) {
                Ok(parsed) => println!("Parsed command: {:?}", parsed),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(("file", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
            match fs::read_to_string(file) {
                Ok(contents) => {
                    for line in contents.lines() {
                        match parse_command(line) {
                            Ok(parsed) => println!("Parsed: {:?}", parsed),
                            Err(e) => eprintln!("Error in line '{}': {}", line, e),
                        }
                    }
                }
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
        Some(("info", _)) => {
            println!("rpg-chat-command-parser: A tool for parsing RPG-style chat commands.\n");
            println!("USAGE:");
            println!("  rpg-chat-command-parser parse <command>");
            println!("  rpg-chat-command-parser file <file>");
            println!("  rpg-chat-command-parser info");
            println!("\nFor detailed documentation, visit: https://github.com/Emril44/rpg-chat-command-parser");
        }
        _ => {
            println!("Use --info for available commands.");
        }
    }
}