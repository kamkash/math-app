use log::info;
use symengine_rs::basic::Basic;

use std::{fmt, rc::Rc};
pub mod asciimath_basic_interpreter;
pub mod asciimath_gen_interpreter;
pub mod asciimath_basic_string_interpreter;
pub mod calc_basic_interpreter;
pub mod calc_basic_string_interpreter;
pub mod latex_basic_interpreter;
pub mod latex_gen_interpreter;

pub struct SymEquation {
    pub left: Rc<Basic>,
    pub right: Rc<Basic>,
    pub relop: Rc<Basic>,
}

impl SymEquation {
    pub fn new(left: Rc<Basic>, right: Rc<Basic>, relop: Rc<Basic>) -> Self {
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

pub trait ExplicitFunction {
    fn generate(&self, args: &[Rc<Basic>]) -> Rc<Basic>;
}

pub struct Logb;
impl ExplicitFunction for Logb {
    fn generate(&self, args: &[Rc<Basic>]) -> Rc<Basic> {
        assert!(
            args.len() == 1,
            "Logb function symbol requires exactly one argument for log base, found: {:?}",
            args.len()
        );
        assert!(
            args[0].is_number(),
            "Expected a number for log base, found: {:?}",
            args[0]
        );
        let base: i64 = args[0].to_f64().unwrap().floor() as i64;
        Rc::new(Basic::logb_func_sym(base))
    }
}

pub fn create_function(name: &str) -> Option<Box<dyn ExplicitFunction>> {
    match name {
        "log" => Some(Box::new(Logb)),
        _ => None,
    }
}

///////////////////////////////////////////////////////////////////////
/// interpreter module.
///////////////////////////////////////////////////////////////////////

pub fn evaluate_ascii_math(input: &str) -> String {
    info!("evaluate_ascii_math {}", input);
    let result = calc_basic_interpreter::evaluate_ascii_math_block(input);
    format!("{}", result.unwrap())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
