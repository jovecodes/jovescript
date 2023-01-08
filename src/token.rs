use std::{fmt::Display, any::Any};

pub const INT: &str = "INT";
pub const FLOAT: &str = "FLOAT";
pub const PLUS: &str = "PLUS";
pub const MINUS: &str = "MINUS";
pub const MUL: &str = "MUL";
pub const DIV: &str = "DIV";
pub const LPAREN: &str = "LPAREN";
pub const RPAREN: &str = "RPAREN";

pub const NUMBERS: &str = "0123456789";
pub const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Token {
    token_type: String,
    value: Option<Box<dyn Any>>,
    string_version: String
}

impl Token {
    pub fn new<T: Display + 'static>(value: T, token_type: &str) -> Self {
        Self { 
            token_type: token_type.to_owned(), 
            string_version: format!("{}", value),
            value: Some(Box::new(value)),
        }
    }

    pub fn without_value(token_type: &str) -> Self {
        Self { 
            token_type: token_type.to_string(), 
            value: None,
            string_version: String::new()
        }
    }

    pub fn get_value<T: Display + 'static>(&mut self) -> &mut T {
        match self.value.as_mut() {
            Some(v) => return v.downcast_mut::<T>().expect("Token downcast was not corrent type"),
            None => unreachable!("Token value is not corrent type"),
        }
    }

    pub fn get(&self) -> String {
        if self.value.is_none() {
            return self.token_type.to_owned();
        }

        return format!("{}:{}", self.token_type, self.string_version);
    }
}
