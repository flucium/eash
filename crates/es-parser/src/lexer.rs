use std::mem;

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    is_eof: bool,
    prev: Option<Token>,
    peek: Option<Token>,
}

impl Lexer {
    pub fn new(string: &str) -> Self {
        Self {
            input: string.chars().collect(),
            position: 0,
            is_eof: false,
            prev: None,
            peek: None,
        }
    }

    pub fn consume(&mut self) {
        self.prev = self.next();
    }

    pub fn next_is(&mut self, token: &Token) -> bool {
        if self.peek.is_none() {
            self.peek();
        }

        match &self.peek {
            Some(peek) => mem::discriminant(peek) == mem::discriminant(token),
            None => false,
        }
    }

    pub fn prev(&mut self) -> Option<&Token> {
        self.prev.as_ref()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peek.is_none() {
            self.peek = self.read();
        }

        self.peek.as_ref()
    }

    fn read(&mut self) -> Option<Token> {
        while let Some(ch) = self.input.get(self.position) {
            if ch.is_whitespace() {
                self.position += 1;
                continue;
            }

            match ch {
                '#' => self.skip_commentout(),

                '\n' => return Some(Token::EOL),

                // pipe ∨ or
                '|' => {
                    if matches!(self.peek_ch(), Some('|')) {
                        self.position += 2;
                        return Some(Token::OR);
                    }

                    self.position += 1;

                    return Some(Token::Pipe);
                }

                // assign ∨ equal
                '=' => {
                    if matches!(self.peek_ch(), Some('=')) {
                        self.position += 2;
                        return Some(Token::Equal);
                    }

                    self.position += 1;

                    return Some(Token::Assign);
                }

                // semicolon
                ';' => {
                    self.position += 1;
                    return Some(Token::Semicolon);
                }

                // comma
                ',' => {
                    self.position += 1;
                    return Some(Token::Comma);
                }

                // bang ∨ notequal
                '!' => {
                    if matches!(self.peek_ch(), Some('=')) {
                        self.position += 2;
                        return Some(Token::NotEqual);
                    }

                    self.position += 1;
                    return Some(Token::Bang);
                }

                // gt
                '>' => {
                    self.position += 1;
                    return Some(Token::Gt);
                }

                // lt
                '<' => {
                    self.position += 1;
                    return Some(Token::Lt);
                }

                // dollar ∨ ident
                '$' => {
                    if let Some(ch) = self.peek_ch() {
                        if ch.is_whitespace() == false {
                            self.position += 1;
                            if let Some(string) = self.read_string(false) {
                                return Some(Token::Ident(string));
                            }
                        }
                    }

                    self.position += 1;
                    return Some(Token::Dollar);
                }

                // ampersand ∨ and ∨ fd
                '&' => {
                    if matches!(self.peek_ch(), Some('&')) {
                        self.position += 2;
                        return Some(Token::AND);
                    }

                    if let Some(ch) = self.peek_ch() {
                        if ch.is_whitespace() == false {
                            self.position += 1;
                            if let Some(n) = self.read_u32() {
                                {
                                    return Some(Token::FD(n));
                                }
                            }
                        }
                    }

                    self.position += 1;
                    return Some(Token::Ampersand);
                }

                // left paren
                '(' => {
                    self.position += 1;
                    return Some(Token::LParen);
                }

                // right paren
                ')' => {
                    self.position += 1;
                    return Some(Token::RParen);
                }

                // left brace
                '{' => {
                    self.position += 1;
                    return Some(Token::LBrace);
                }

                // right brace
                '}' => {
                    self.position += 1;
                    return Some(Token::RBrace);
                }

                '"' => {
                    self.position += 1;

                    match self.read_string(true) {
                        Some(string) => return Some(Token::String(string)),
                        None => break,
                    }
                }

                _ => {
                    if let Some(number) = self.read_number() {
                        return Some(Token::Number(number));
                    }

                    if let Some(string) = self.read_string(false) {
                        match string.as_str() {
                            "loop" => return Some(Token::Loop),
                            "if" => return Some(Token::If),
                            "elif" => return Some(Token::Elif),
                            "else" => return Some(Token::Else),
                            "def" => return Some(Token::Def),
                            "return" => return Some(Token::Return),
                            _ => return Some(Token::String(string)),
                        }
                    }
                }
            }
        }

        match self.is_eof {
            true => None,
            false => {
                self.is_eof = true;
                Some(Token::EOF)
            }
        }
    }

    fn read_u32(&mut self) -> Option<u32> {
        let origin = self.position;

        match self.read_string(false)?.parse::<u32>() {
            Ok(n) => Some(n),
            Err(_) => {
                self.position = origin;
                None
            }
        }
    }

    // fn read_i32(&mut self) -> Option<i32> {
    //     let origin = self.position;

    //     match self.read_string(false)?.parse::<i32>() {
    //         Ok(n) => Some(n),
    //         Err(_) => {
    //             self.position = origin;
    //             None
    //         }
    //     }
    // }

    fn read_number(&mut self) -> Option<isize> {
        let origin = self.position;

        match self.read_string(false)?.parse::<isize>() {
            Ok(n) => Some(n),
            Err(_) => {
                self.position = origin;
                None
            }
        }
    }

    fn read_string(&mut self, esc: bool) -> Option<String> {
        let mut string = String::new();

        while let Some(ch) = self.input.get(self.position) {
            if esc {
                if ch == &'"' {
                    self.position += 1;
                    break;
                }
            } else {
                if ch.is_whitespace()
                    || matches!(
                        ch,
                        ';' | ',' | '=' | '|' | '>' | '<' | '(' | ')' | '{' | '}'
                    )
                {
                    break;
                }
            }

            self.position += 1;
            string.push(*ch);
        }

        if string.is_empty() {
            None
        } else {
            Some(string)
        }
    }

    fn peek_ch(&self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn skip_commentout(&mut self) {
        if matches!(self.input.get(self.position), Some('#')) == false {
            return;
        }

        while let Some(ch) = self.input.get(self.position) {
            self.position += 1;
            if ch == &'\n' {
                break;
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.peek.take() {
            self.prev = Some(token.clone());
            return Some(token);
        }

        let token = self.read();

        self.prev = token.clone();

        token
    }
}
