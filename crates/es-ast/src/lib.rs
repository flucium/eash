#[derive(Debug)]
pub enum Statement {
    Loop(Loop),
    If(If),
    Assign(Assign),
    Block(Block),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Loop {
    comparison: Option<Comparison>,
    block: Option<Block>,
}

#[derive(Debug)]
pub struct If {
    comparison: Option<Comparison>,
    block: Option<Block>,
    child: Option<Box<Self>>,
}

#[derive(Debug)]
pub struct Assign {
    left: Option<Expression>,
    right: Option<Expression>,
}

#[derive(Debug)]
pub struct Block {
    left: Option<Box<Statement>>,
    right: Option<Box<Statement>>,
}

#[derive(Debug)]
pub enum Expression {
    String(String),
    Variable(String),
    Number(isize),
    FD(i32),
    Command(Command),
    Redirect(Redirect),
    Pipe(Pipe),
    Comparison(Comparison),
}

#[derive(Debug)]
pub struct Command {
    prefix: Option<Box<Expression>>,
    suffix: Option<CommandSuffix>,
}

#[derive(Debug)]
pub struct CommandSuffix {
    expr: Option<Box<Expression>>,
    suffix: Option<Box<Self>>,
}

#[derive(Debug)]
pub struct Redirect {
    kind: RedirectKind,
    left: Option<Box<Expression>>,
    right: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum RedirectKind {
    Write,
    Read,
}

#[derive(Debug)]
pub struct Pipe {
    left: Option<Box<Expression>>,
    right: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum Comparison {
    Equal {
        left: Option<Box<Expression>>,
        right: Option<Box<Expression>>,
    },
    NotEqual {
        left: Option<Box<Expression>>,
        right: Option<Box<Expression>>,
    },
    Gt {
        left: Option<Box<Expression>>,
        right: Option<Box<Expression>>,
    },
    Lt {
        left: Option<Box<Expression>>,
        right: Option<Box<Expression>>,
    },
}

