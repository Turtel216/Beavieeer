// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

pub type Program = Vec<Statement>;

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    LetStmt(Ident, Expression),
    ReturnStmt(Expression),
    ExprStmt(Expression),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    IdentExpr(Ident),
    LitExpr(Literal),
    PrefixExpr(Prefix, Box<Expression>),
    InfixExpr(Infix, Box<Expression>, Box<Expression>),
    IfExpr {
        cond: Box<Expression>,
        consequence: Program,
        alternative: Option<Program>,
    },
    FnExpr {
        params: Vec<Ident>,
        body: Program,
    },
    CallExpr {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    ArrayExpr(Vec<Expression>),
    HashExpr(Vec<(Literal, Expression)>),
    IndexExpr {
        array: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {
    PrefixPlus,
    PrefixMinus,
    Not,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    PLowest,
    PEquals,
    PLessGreater,
    PSum,
    PProduct,
    PCall,
    PIndex,
}
