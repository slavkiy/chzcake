use crate::{
    ast::Literal,
    pattern::{self, Pattern},
    stmt::Stmt,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Match(Match),

    // name
    Name(String),

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Postfix {
        expr: Box<Expr>,
        op: PostfixOp,
    },

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    Call {
        name: Box<Expr>, // Name(String)::Name(String)::Name(String).Name(String)
        args: Option<Box<Vec<pattern::Pattern>>>,
        generic: Option<Box<Vec<Expr>>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg, // -
    Not, // !
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOp {
    Increment, // ++
    Decrement, // --
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /

    Eq, // ==
    Ne, // !=

    Lt, // <
    Gt, // >

    And, // &&
    Or,  // |
}

impl BinaryOp {
    pub fn precedence(self) -> u8 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,

            BinaryOp::Eq | BinaryOp::Ne => 3,

            BinaryOp::Lt | BinaryOp::Gt => 4,

            BinaryOp::Add | BinaryOp::Sub => 5,

            BinaryOp::Mul | BinaryOp::Div => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub expr: Box<Expr>,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Box<Stmt>,
}
