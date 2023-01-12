pub mod error;
pub mod lexer;
pub mod token;
use error::*;
use es_ast::*;
use lexer::*;
use token::*;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer: lexer }
    }

    pub fn parse(&mut self) {}

    pub fn parse_assign(&mut self) {}

    pub fn parse_command(&mut self) -> Result<Command> {
        let prefix = match self.parse_command_prefix() {
            Err(err) => Err(err)?,
            Ok(prefix) => prefix,
        };

        let mut command = Command::new(prefix);

        match self.parse_command_suffix() {
            Err(err) => Err(err)?,
            Ok(ok) => {
                if let Some(suffix) = ok {
                    command.insert_suffix(suffix);
                }
            }
        }

        Ok(command)
    }

    fn parse_command_suffix(&mut self) -> Result<Option<CommandSuffix>> {
        // return if token is Pipe || Semicolon || EOL || EOF
        if matches!(
            self.lexer.peek(),
            None | Some(Token::Pipe) | Some(Token::Semicolon) | Some(Token::EOL) | Some(Token::EOF)
        ) {
            return Ok(None);
        }

        let mut suffix = CommandSuffix::new();

        while let Some(token) = self.lexer.next() {
            if let Ok(fd) = parse_fd(&token) {
                match self.lexer.peek() {
                    Some(Token::Gt) => {
                        self.lexer.consume();

                        match self.lexer.next() {
                            None => Err(Error::new(ErrorKind::Unknown, "".to_owned()))?,
                            Some(token) => {
                                match parse_string(&token)
                                    .or(parse_number(&token).or(parse_fd(&token)))
                                {
                                    Err(err) => Err(err)?,
                                    Ok(expr) => {
                                        suffix.insert(Expression::Redirect(Redirect::new(
                                            RedirectKind::Write,
                                            fd,
                                            expr,
                                        )));
                                    }
                                }
                            }
                        }
                    }
                    Some(Token::Lt) => {
                        self.lexer.consume();

                        match self.lexer.next() {
                            None => Err(Error::new(ErrorKind::Unknown, "".to_owned()))?,
                            Some(token) => {
                                match parse_string(&token)
                                    .or(parse_number(&token).or(parse_fd(&token)))
                                {
                                    Err(err) => Err(err)?,
                                    Ok(expr) => {
                                        suffix.insert(Expression::Redirect(Redirect::new(
                                            RedirectKind::Read,
                                            fd,
                                            expr,
                                        )));
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        suffix.insert(fd);
                    }
                }
            } else {
                if let Ok(expr) = parse_variable(&token) {
                    suffix.insert(expr);
                } else {
                    match token {
                        Token::Ampersand => {
                            suffix.insert(Expression::Background(true));
                            break;
                        }

                        _ => {
                            suffix.insert(Expression::String(token.to_string()));
                        }
                    }
                }
            }

            if self.lexer.next_is(&Token::Pipe)
                || self.lexer.next_is(&Token::Semicolon)
                || self.lexer.next_is(&Token::EOL)
                || self.lexer.next_is(&Token::EOF)
            {
                break;
            }
        }

        Ok(Some(suffix))
    }

    fn parse_command_prefix(&mut self) -> Result<Expression> {
        match self.lexer.peek() {
            None => Err(Error::new(ErrorKind::Unknown, "".to_owned()))?,

            Some(token) => {
                let expr = parse_variable(token).or(parse_string(token).or(parse_number(token)));

                match expr {
                    Err(err) => Err(err),

                    Ok(expr) => {
                        self.lexer.consume();

                        Ok(expr)
                    }
                }
            }
        }
    }
}

// fn next_token_is_eof(lexer: &mut Peekable<Lexer>) -> bool {
//     lexer.peek() == Some(&Token::EOF)
// }

// fn next_token_is_eol(lexer: &mut Peekable<Lexer>) -> bool {
//     lexer.peek() == Some(&Token::EOL)
// }

// fn next_token_is_semicolon(lexer: &mut Peekable<Lexer>) -> bool {
//     lexer.peek() == Some(&Token::Semicolon)
// }

// fn next_token_is_pipe(lexer: &mut Peekable<Lexer>) -> bool {
//     lexer.peek() == Some(&Token::Pipe)
// }

fn parse_fd(token: &Token) -> Result<Expression> {
    match token {
        Token::FD(fd) => Ok(Expression::FD(fd.to_owned())),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_number(token: &Token) -> Result<Expression> {
    match token {
        Token::Number(number) => Ok(Expression::Number(number.to_owned())),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_variable(token: &Token) -> Result<Expression> {
    match token {
        Token::Ident(string) => Ok(Expression::Variable(string.to_owned())),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_string(token: &Token) -> Result<Expression> {
    match token {
        Token::String(string) => Ok(Expression::String(string.to_owned())),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

// #[macro_export]
// macro_rules! parse_assign {
//     ($($x:tt)*) => {};
// }

// #[macro_export]
// macro_rules! parse_command {

//     ($($x:tt)*)=>{{

//         if $($x)*.is_empty(){
//             return
//         }

//         let mut lexer = $crate::lexer::Lexer::new($($x)*);

//         let prefix=match lexer.next(){
//             Some($crate::token::Token::String(string))=>{
//                 es_ast::Expression::String(string.to_owned())
//             }

//             Some($crate::token::Token::Ident(ident))=>{
//                 es_ast::Expression::Variable(ident.to_owned())
//             }

//             Some($crate::token::Token::Number(number))=>{
//                 es_ast::Expression::Number(number.to_owned())
//             }

//             Some($crate::token::Token::EOL)|Some($crate::token::Token::EOF)=>{
//                 return
//             }

//             _=>{
//                 panic!("")
//             }
//         };

//         let mut command = es_ast::Command::new(prefix);

//         while let Some(token) = lexer.next(){
//             match token{
//                 $crate::token::Token::String(string) => {
//                     command.insert_suffix(es_ast::Expression::String(string.to_owned()));
//                 }
//                 $crate::token::Token::Ident(ident) => {
//                     command.insert_suffix(es_ast::Expression::Variable(ident.to_owned()));
//                 }
//                 $crate::token::Token::Number(number) => {
//                     command.insert_suffix(es_ast::Expression::Number(number.to_owned()));
//                 }
//                 $crate::token::Token::Ampersand => {
//                     command.insert_suffix(es_ast::Expression::Background(true));
//                     break;
//                 }

//                 $crate::token::Token::FD(fd)=>{}

//                 $crate::token::Token::Gt=>{}

//                 $crate::token::Token::Lt=>{}

//                 $crate::token::Token::Semicolon | $crate::token::Token::Pipe | $crate::token::Token::EOL | $crate::token::Token::EOF => break,

//                 _=> {
//                     panic!("")
//                 }
//             }
//         }

//         es_ast::Expression::Command(command)
//     }};
// }
