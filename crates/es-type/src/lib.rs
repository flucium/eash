#[derive(Debug,PartialEq)]
pub enum Type {
    String(String),
    Boolean(bool),
    Number(isize),
    FD(i32),
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::String(s) => s.to_string(),
            Type::Boolean(b) => b.to_string(),
            Type::Number(n) => n.to_string(),
            Type::FD(fd) => fd.to_string(),
        }
    }
}
