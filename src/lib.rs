#![warn(missing_docs)]

//! A library for handling algebra.

#[derive(Debug)]
pub enum Combinator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug)]
pub enum Function {
    Root,
    LogN,
}

#[derive(Debug)]
pub enum Comparator {
    Eq,
    NE,
    GT,
    LT,
    GE,
    LE,
}
