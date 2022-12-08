use crate::token::Token;
use std::collections::VecDeque;
pub struct Lexer {
    input: VecDeque<char>,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
        }
    }

    fn pop_front(&mut self) -> Option<Token> {
        while let Some(ch) = self.input.pop_front() {
            if ch.is_whitespace() {
                continue;
            }

            match ch {
                '#' => {
                    while let Some(ch) = self.input.pop_front() {
                        if ch == '\n' {
                            break;
                        }
                    }
                }
                ';' => return Some(Token::Semicolon),
                '=' => return Some(Token::Equal),
                '|' => return Some(Token::Pipe),
                '>' => return Some(Token::Gt),
                '<' => return Some(Token::Lt),
                '&' => return Some(Token::Ampersand),
                '$' => return Some(Token::Dollar),
                '"' => return Some(Token::String(self.read_string(true))),
                // ch @ '0'..='9' => {

                // }
                _ => {
                    let mut string = String::from(ch);

                    string.push_str(&self.read_string(false));

                    return Some(match string.parse::<isize>() {
                        Ok(n) => Token::Number(n),
                        Err(_) => Token::String(string),
                    });
                }
            }
        }

        None
    }

    fn read_string(&mut self, esc: bool) -> String {
        let mut string_buffer = String::new();

        while let Some(ch) = self.input.pop_front() {
            if esc {
                if ch == '"' {
                    break;
                }
            } else {
                if ch.is_whitespace() || matches!(ch, ';' | '=' | '|' | '>' | '<') {
                    self.input.push_front(ch);
                    break;
                }
            }

            string_buffer.push(ch);
        }

        string_buffer
    }
}
