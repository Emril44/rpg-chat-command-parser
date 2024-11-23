use clap::{Arg, Command};
use std::{fs, io};
use rpg_chat_command_parser::{parse_command, ParsedCommand};

fn main() {
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
        .get_matches();

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
            _ => {
                println!("Use --help for available commands.");
            }
        }
}