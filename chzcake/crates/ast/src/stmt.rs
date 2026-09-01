use crate::{expr::Expr, typ::Typ};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Var(Var),
}

// let x: T := expr
// const x: T := expr
// mut x: T := expr
#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub name: String,
    pub typ: Option<Box<Typ>>,
    pub value: Option<Box<Expr>>,
    pub is_mutable: bool,
    pub is_comptime: bool,
}
