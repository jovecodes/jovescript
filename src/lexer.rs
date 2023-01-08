use std::convert::TryInto;

use crate::{token::*, position_tracker::Position};


pub struct Lexer {
    text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let mut lexer = Self { 
            text, 
            pos: Position::new(),
            current_char: None,
        };
        lexer.update_char();
        lexer
    }

    pub fn make_tokens(&mut self, file_name: String) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current_char.is_some() {
            match self.current_char.unwrap() {
                '+' => self.push_token(PLUS, &mut tokens),
                '-' => self.push_token(MINUS, &mut tokens),
                '*' => self.push_token(MUL, &mut tokens),
                '/' => self.push_token(DIV, &mut tokens),
                '(' => self.push_token(LPAREN, &mut tokens),
                ')' => self.push_token(RPAREN, &mut tokens),
                _ => {
                    if self.is_number() {
                        self.add_number(&mut tokens);
                    } else if self.is_word() {
                        self.advance();
                    } else if self.is_normal_char() {
                        self.advance()
                    } else {
                        match self.current_char {
                            None => panic!("Error current char is null"),
                            Some(c) => unreachable!(
                                "unknown character '{}' in file '{}' at position {}", 
                                c, 
                                file_name,
                                self.pos.get_position()
                            )
                        }
                    }
                }
            }
        }
        tokens
    }

    fn update_char(&mut self) {
        if self.pos.idx >= self.text.len().try_into().unwrap() {
            self.current_char = None;
        } else {
            self.current_char = self.text.chars().nth(self.pos.idx);
        }
    }

    fn advance(&mut self) {
        self.pos.advance(self.current_char.unwrap());
        self.update_char()
    }

    pub fn push_token(&mut self, token_type: &str, tokens: &mut Vec<Token>) {
        tokens.push(Token::without_value(token_type));
        self.advance();
    }

    fn is_number(&self) -> bool {
        NUMBERS.contains(self.current_char.unwrap()) 
    }

    fn add_number(&mut self, tokens: &mut Vec<Token>) {
        let mut number = String::new();
        let mut dot_count = 0;

        loop {
            if self.current_char == Some('.') {
                dot_count += 1;
                if dot_count > 1 {
                    break;
                }
            }
            number.push(self.current_char.unwrap());
            self.advance();

            if self.current_char.is_none() { break; }
            if (NUMBERS.to_string() + ".").contains(self.current_char.unwrap()) == false { break; }
        }

        if dot_count > 0 {
            tokens.push(Token::new(number, FLOAT));
        } else {
            tokens.push(Token::new(number, INT))
        }
    }

    fn is_word(&self) -> bool {
        LETTERS.contains(self.current_char.unwrap()) 
    }

    fn is_normal_char(&self) -> bool {
        match self.current_char {
            Some(' ') => return true,
            Some('\n') => return true,
            Some('\t') => return true,
            _ => return false,
        }
    }
}

