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
    comparison: Comparison,
    block: Block,
}

#[derive(Debug)]
pub struct If {
    comparison: Comparison,
    block: Block,
    child: Option<Box<Self>>,
}

#[derive(Debug)]
pub struct Assign {
    left: Expression,
    right: Expression,
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
    prefix: Box<Expression>,
    suffix: Option<CommandSuffix>,
}

#[derive(Debug)]
pub struct CommandSuffix {
    expr: Box<Expression>,
    suffix: Option<Box<Self>>,
}

#[derive(Debug)]
pub struct Redirect {
    kind: RedirectKind,
    left: Box<Expression>,
    right: Box<Expression>,
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
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Lt {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}
