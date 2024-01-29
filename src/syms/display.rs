use std::fmt::{write, Display};

use super::Constant;
use super::Operation;

use super::Sym;

impl Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", *n),
            Self::Identifier(i) => write!(f, "{}", *i),
            Self::Constant(c) => write!(f, "{}", *c),
            Self::Operation(op) => write!(f, "{}", *op),
        }
    }
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sin(s) => write!(f, "sin({})", s),
            Self::Cos(s) => write!(f, "cos({})", s),
            Self::Rem(s1, s2) => write!(f, "{s1}%{s2}"),
            Self::Mul(s1, s2) => write!(f, "({}*{})", s1, s2),
            Self::Div(s1, s2) => write!(f, "({}/{})", s1, s2),
            Self::Sub(s1, s2) => write!(f, "({}-{})", s1, s2),
            Self::Add(s1, s2) => write!(f, "({}+{})", s1, s2),
            Self::Sqrt(s) => write!(f, "sqrt({})", s),
            Self::UnSub(s) => write!(f, "-{s}"),
        }
    }
}
impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pi => write!(f, "\\pi"),
        }
    }
}
