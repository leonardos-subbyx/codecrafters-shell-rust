#![allow(dead_code, unused_imports)]

use commands::{parse_command, ParseCommandError, ParseCommandResult};
use std::io::{self, Write};

mod commands;
mod path;

fn eval(input: &str) -> Option<String> {
    let trimmed_input = input.trim_end_matches("\n");

    let ParseCommandResult {
        command_name,
        command,
    } = parse_command(trimmed_input);
    match command {
        Err(ParseCommandError(message)) => Some(format!("{}: {}", &command_name, &message)),
        Ok(command) => command.execute(),
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if input.len() > 1 {
            let output = eval(&input);
            if let Some(output) = output {
                println!("{}", output);
            }
        }
    }
}
