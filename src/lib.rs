//! # RPG Chat Command Parser
//!
//! This binary provides a command-line interface (CLI) for parsing RPG-style chat commands.
//!
//! ## Usage
//!
//! You can run the binary with the following subcommands:
//!
//! ### Parse a Single Command
//! ```sh
//! rpg-chat-command-parser parse "<command>"
//! ```
//! Example:
//! ```sh
//! rpg-chat-command-parser parse "/cast fireball --power=high"
//! ```
//!
//! ### Parse Commands from a File
//! ```sh
//! rpg-chat-command-parser file "<file_path>"
//! ```
//! Example:
//! ```sh
//! rpg-chat-command-parser file commands.txt
//! ```
//!
//! ### Display Additional Help Information
//! ```sh
//! rpg-chat-command-parser info
//! ```
//!
//! ## Installation
//! - Clone the repository: `git clone https://github.com/Emril44/rpg-chat-command-parser.git`
//! - Build the project: `cargo build --release`
//! - Run the binary: `./target/release/rpg-chat-command-parser`
pub mod errors;
pub mod parser;

pub use errors::CommandError;
pub use parser::{parse_command, ParsedCommand};
