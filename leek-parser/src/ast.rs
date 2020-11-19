use std::fmt::Display;

use peg::str::*;
use peg::Parse;

#[derive(Debug)]
pub enum EXPR {
    Cst(usize, u32),
    Var(usize, String),
    ECall(usize, String, Vec<EXPR>),
}

#[derive(Debug)]
pub enum STMT {
    Declr(usize, String, EXPR),
    Affect(usize, String, EXPR),
    If(usize, EXPR, Vec<STMT>, Option<Vec<STMT>>),
    While(usize, EXPR, Vec<STMT>),
    Defun(usize, String, Vec<Box<str>>, Vec<STMT>),
    Call(usize, String, Vec<EXPR>),
    Return(usize, EXPR),
}

pub trait StringLocated {
    fn line_col(&self, inp: &str) -> LineCol;
}

impl StringLocated for STMT {
    fn line_col(&self, inp: &str) -> LineCol {
        match *self {
            STMT::Declr(p, _, _) => (*inp).position_repr(p),
            STMT::Affect(p, _, _) => (*inp).position_repr(p),
            STMT::If(p, _, _, _) => (*inp).position_repr(p),
            STMT::While(p, _, _) => (*inp).position_repr(p),
            STMT::Defun(p, _, _, _) => (*inp).position_repr(p),
            STMT::Call(p, _, _) => (*inp).position_repr(p),
            STMT::Return(p, _) => (*inp).position_repr(p),
        }
    }
}

pub struct SPROG<'a> {
    text: String,
    s: &'a STMT,
}

impl<'a> SPROG<'a> {
    pub fn new(text: String, s: &'a STMT) -> SPROG {
        SPROG { text, s }
    }
}

impl<'a> Display for SPROG<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.text.as_str();
        let stmt: &STMT = &self.s;
        match stmt {
            STMT::Declr(_, a, b) => writeln!(f, "{} Declr({}, {:?})", stmt.line_col(text), a, b),
            STMT::Affect(_, a, b) => writeln!(f, "{} Affect({}, {:?})", stmt.line_col(text), a, b),
            STMT::If(_, c, a, b) => {
                writeln!(f, "{} If({:?}, {:?}, {:?})", stmt.line_col(text), c, a, b)
            }
            STMT::While(_, c, a) => writeln!(f, "{} While({:?}, {:?})", stmt.line_col(text), c, a),
            STMT::Defun(_, fun, a, b) => writeln!(
                f,
                "{} Defun({}, {:?}, {:?})",
                stmt.line_col(text),
                fun,
                a,
                b
            ),
            STMT::Call(_, fun, a) => writeln!(f, "{} Call({}, {:?})", stmt.line_col(text), fun, a),
            STMT::Return(_, e) => writeln!(f, "{} Return {:?})", stmt.line_col(text), e),
        }
    }
}
