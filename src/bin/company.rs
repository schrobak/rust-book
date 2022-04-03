use book::company::{parse_command, Command};
use std::io::stdin;

fn read_line() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error reading word from stdin");
    let input = input.trim();
    String::from(input)
}

pub fn main() {
    println!("Company employees management");
    loop {
        let command = read_line();
        match parse_command(&command) {
            Command::None => println!("No command provided"),
            Command::Unknown(cmd) => println!("Unknown command {}", cmd),
            _ => (),
        }
    }
}
