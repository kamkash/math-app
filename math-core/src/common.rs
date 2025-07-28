#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinFunction {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Csc,
    Sec,
    Exp,
    Log,
    Logb,
    Ln, // natural logarithm
    Sqrt,
}
impl BuiltinFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            BuiltinFunction::Sin => "sin",
            BuiltinFunction::Cos => "cos",
            BuiltinFunction::Tan => "tan",
            BuiltinFunction::Asin => "asin",
            BuiltinFunction::Acos => "acos",
            BuiltinFunction::Atan => "atan",
            BuiltinFunction::Csc => "csc",
            BuiltinFunction::Sec => "sec",
            BuiltinFunction::Exp => "exp",
            BuiltinFunction::Log => "log",
            BuiltinFunction::Ln => "ln",
            BuiltinFunction::Sqrt => "sqrt",
            BuiltinFunction::Logb => "logb",
        }
    }
}
impl BuiltinFunction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sin" => Some(BuiltinFunction::Sin),
            "cos" => Some(BuiltinFunction::Cos),
            "tan" => Some(BuiltinFunction::Tan),
            "asin" => Some(BuiltinFunction::Asin),
            "acos" => Some(BuiltinFunction::Acos),
            "atan" => Some(BuiltinFunction::Atan),
            "csc" => Some(BuiltinFunction::Csc),
            "sec" => Some(BuiltinFunction::Sec),
            "exp" => Some(BuiltinFunction::Exp),
            "log" => Some(BuiltinFunction::Log),
            s if s.contains("log_") => Some(BuiltinFunction::Logb),
            "ln" => Some(BuiltinFunction::Ln),
            "sqrt" => Some(BuiltinFunction::Sqrt),
            _ => None,
        }
    }
}


// Operator enum for idiomatic Rust usage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::Add => "__ADD__",
            Operator::Sub => "__SUB__",
            Operator::Mul => "__MUL__",
            Operator::Div => "__DIV__",
            Operator::Pow => "__POW__",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "__ADD__" => Some(Operator::Add),
            "__SUB__" => Some(Operator::Sub),
            "__MUL__" => Some(Operator::Mul),
            "__DIV__" => Some(Operator::Div),
            "__POW__" => Some(Operator::Pow),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogicalOperator {
    Eq,       // '='
    DoubleEq, // '=='
    Neq,      // '!=' | '<>'
    Lt,       // '<'
    Gt,       // '>'
    Lte,      // '<=' | 'le'
    Gte,      // '>=' | 'ge'
}

impl LogicalOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogicalOperator::Eq => "__EQ__",
            LogicalOperator::DoubleEq => "__EQEQ__",
            LogicalOperator::Neq => "__NEQ__", // canonical string
            LogicalOperator::Lt => "__LT__",
            LogicalOperator::Gt => "__GT__",
            LogicalOperator::Lte => "__LE__",
            LogicalOperator::Gte => "__GE__",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "__EQ__" => Some(LogicalOperator::Eq),
            "__EQEQ__" => Some(LogicalOperator::DoubleEq),
            "__NE__" => Some(LogicalOperator::Neq),
            "__LT__" => Some(LogicalOperator::Lt),
            "__GT__" => Some(LogicalOperator::Gt),
            "__LE__" => Some(LogicalOperator::Lte),
            "__GE__" => Some(LogicalOperator::Gte),
            _ => None,
        }
    }

    pub fn from_str_token(s: &str) -> Option<Self> {
        match s {
            "=" => Some(LogicalOperator::Eq),
            "==" => Some(LogicalOperator::DoubleEq),
            "!=" | "<>" => Some(LogicalOperator::Neq),
            "<" => Some(LogicalOperator::Lt),
            ">" => Some(LogicalOperator::Gt),
            "<=" | "le" => Some(LogicalOperator::Lte),
            ">=" | "ge" => Some(LogicalOperator::Gte),
            _ => None,
        }
    }
}
