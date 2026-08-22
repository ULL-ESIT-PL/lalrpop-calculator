use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, OpCode, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::Op(lhs, op, rhs) => write!(f, "{op}({lhs}, {rhs})"),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            OpCode::Mul => "Mul",
            OpCode::Div => "Div",
            OpCode::Add => "Add",
            OpCode::Sub => "Sub",
        };
        write!(f, "{symbol}")
    }
}