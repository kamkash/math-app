use log::info;
use symengine_rs::basic::Basic;

use std::{fmt, rc::Rc};
pub mod gen_calc_parser;
pub mod string_evaluator;
pub mod symengine_evaluator;

pub struct SymEquation {
    pub left: Rc<Basic>,
    pub right: Rc<Basic>,
    pub relop: Relop,
}

impl SymEquation {
    pub fn new(left: Rc<Basic>, right: Rc<Basic>, relop: Relop) -> Self {
        SymEquation { left, right, relop }
    }
}

impl std::fmt::Debug for SymEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.left, self.relop, self.right)
    }
}

impl std::fmt::Display for SymEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.relop, self.right)
    }
}

pub enum ArithmeticOp {
    Lparen,
    Rparen,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Times,
    DivideBy,
    PowerBy,
    Modulus,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
}

impl fmt::Display for ArithmeticOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ArithmeticOp::Lparen => "(",
            ArithmeticOp::Rparen => ")",
            ArithmeticOp::Comma => ",",
            ArithmeticOp::Semicolon => ";",
            ArithmeticOp::Plus => "+",
            ArithmeticOp::Minus => "-",
            ArithmeticOp::Times => "*",
            ArithmeticOp::DivideBy => "/",
            ArithmeticOp::PowerBy => "^",
            ArithmeticOp::Modulus => "%",
            ArithmeticOp::BitwiseAnd => "&",
            ArithmeticOp::BitwiseOr => "|",
            ArithmeticOp::BitwiseXor => "^|",
            ArithmeticOp::BitwiseNot => "~",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Debug for ArithmeticOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl From<&str> for ArithmeticOp {
    fn from(s: &str) -> Self {
        match s {
            "(" => ArithmeticOp::Lparen,
            ")" => ArithmeticOp::Rparen,
            "," => ArithmeticOp::Comma,
            ";" => ArithmeticOp::Semicolon,
            "+" => ArithmeticOp::Plus,
            "-" => ArithmeticOp::Minus,
            "*" => ArithmeticOp::Times,
            "/" => ArithmeticOp::DivideBy,
            "^" => ArithmeticOp::PowerBy,
            "%" => ArithmeticOp::Modulus,
            "&" => ArithmeticOp::BitwiseAnd,
            "|" => ArithmeticOp::BitwiseOr,
            "^|" => ArithmeticOp::BitwiseXor,
            "~" => ArithmeticOp::BitwiseNot,
            _ => panic!("Unknown arithmetic operator: {}", s),
        }
    }
}

pub enum Relop {
    Equal,
    DoubleEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl From<&str> for Relop {
    fn from(s: &str) -> Self {
        match s {
            "=" => Relop::Equal,
            "==" => Relop::DoubleEqual,
            "!=" => Relop::NotEqual,
            "<" => Relop::LessThan,
            ">" => Relop::GreaterThan,
            "<=" => Relop::LessThanOrEqual,
            ">=" => Relop::GreaterThanOrEqual,
            _ => panic!("Unknown relational operator: {}", s),
        }
    }
}

impl fmt::Debug for Relop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Relop::DoubleEqual => "==",
            Relop::Equal => "=",
            Relop::NotEqual => "!=",
            Relop::LessThan => "<",
            Relop::GreaterThan => ">",
            Relop::LessThanOrEqual => "<=",
            Relop::GreaterThanOrEqual => ">=",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Relop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn evaluate_ascii_math(input: &str) -> String {
    info!("evaluate_ascii_math {}", input);
    let result = symengine_evaluator::evaluate_ascii_math(input);
    format!("{}", result)
}

pub fn echo_parser(name: &str) -> String {
    info!("parser echo {}", name);
    format!("{}", name)
}
