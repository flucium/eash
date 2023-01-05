pub mod error;
pub mod lexer;
pub mod token;
use error::*;
use es_ast::*;
use lexer::*;
use std::iter::Peekable;
use token::*;

pub fn parse_command(tokens: &[Token]) -> Result<Expression> {
    let token = match tokens.get(0) {
        Some(token) => token,
        None => Err(Error::new(ErrorKind::Unknown, "".to_owned()))?,
    };

    let prefix = match parse_string(token)
        .or(parse_number(token))
        .or(parse_variable(token))
    {
        Ok(prefix) => prefix,
        Err(err) => Err(err)?,
    };

    let mut command = Command::new(prefix);

    tokens[1..].iter().for_each(|token| {
        if let Ok(expr) = parse_string(token)
            .or(parse_number(token))
            .or(parse_variable(token))
        {
            command.insert_suffix(expr);
        } else {
            panic!("")
        }
    });

    Ok(Expression::Command(command))
}

fn parse_variable(token: &Token) -> Result<Expression> {
    match token {
        Token::Ident(ident) => Ok(Expression::Variable(ident.to_owned())),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_string(token: &Token) -> Result<Expression> {
    match token {
        Token::String(string) => Ok(Expression::String(string.to_owned())),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_number(token: &Token) -> Result<Expression> {
    match token {
        Token::Number(number) => Ok(Expression::Number(number.to_owned())),
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_close_fd(token: &Token) -> Result<Expression> {
    match token {
        Token::FD(fd) => {
            if fd < &0 {
                Ok(es_ast::Expression::FD(fd.to_owned()))
            } else {
                Err(Error::new(ErrorKind::Unknown, "".to_owned()))
            }
        }
        _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
    }
}

fn parse_fd(token: &Token) -> Result<Expression> {
    match token {
        Token::FD(fd) => {
            if fd >= &0 {
                Ok(es_ast::Expression::FD(fd.to_owned()))
            } else {
                Err(Error::new(ErrorKind::Unknown, "".to_owned()))
            }
        }
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
