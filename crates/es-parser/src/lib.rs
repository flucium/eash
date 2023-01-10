pub mod error;
pub mod lexer;
pub mod token;
use error::*;
use es_ast::*;
use lexer::*;
use std::iter::Peekable;
use std::mem;
use token::*;

pub struct Parser {
    lexer: Peekable<Lexer>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }
}

fn parse_fd<F>(f: F) -> Result<Expression>
where
    F: FnOnce() -> Option<Token>,
{
    match f() {
        Some(Token::FD(number)) => Ok(Expression::FD(number)),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_number<F>(f: F) -> Result<Expression>
where
    F: FnOnce() -> Option<Token>,
{
    match f() {
        Some(Token::Number(number)) => Ok(Expression::Number(number)),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_variable<F>(f: F) -> Result<Expression>
where
    F: FnOnce() -> Option<Token>,
{
    match f() {
        Some(Token::Ident(string)) => Ok(Expression::Variable(string)),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_string<F>(f: F) -> Result<Expression>
where
    F: FnOnce() -> Option<Token>,
{
    match f() {
        Some(Token::String(string)) => Ok(Expression::String(string)),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn next_token_if_eq(lexer: &mut Peekable<Lexer>, token: Token) -> Option<Token> {
    lexer.next_if(|tkn| mem::discriminant(tkn) == mem::discriminant(&token))
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
