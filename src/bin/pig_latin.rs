use book::pig_latin::to_pig_latin;
use std::io::stdin;

fn read_word() -> String {
    let mut word = String::new();
    stdin()
        .read_line(&mut word)
        .expect("Error reading word from stdin");
    word
}

pub fn main() {
    loop {
        println!("Write the word to convert:");
        let word = read_word();
        let ordway = to_pig_latin(&word);
        println!("Pig latin word: {}", ordway);
    }
}
