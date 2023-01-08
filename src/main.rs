use std::io::{self, Write};

use lexer::Lexer;
use token::Token;

pub mod token;
pub mod lexer;
pub mod position_tracker;

fn main() {
    loop {
        let input = get_input();
        if input == String::from("quit") {
            break;
        }
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.make_tokens("No file".to_owned());

        print_tokens(tokens)
    }
}

fn get_input() -> String {
    let mut input = String::new();
    print!("jovescript > ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_owned();
    input
}

fn print_tokens(tokens: Vec<Token>) {
    print!("Tokens: ");
    for i in tokens {
        print!("{} ", i.get());
    }
    io::stdout().flush().expect("Failed to flush stdout");
    println!("");
}
