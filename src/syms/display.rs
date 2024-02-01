use std::fmt::Display;

use crate::decore::decorators::ToTex;

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
            Self::Prod(els) => write!(
                f,
                "({})",
                els.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join("*")
            ),
            Self::Div(s1, s2) => write!(f, "({}/{})", s1, s2),
            Self::Sub(s1, s2) => write!(f, "({}-{})", s1, s2),
            Self::Add(s1, s2) => write!(f, "({}+{})", s1, s2),
            Self::Sum(els) => write!(
                f,
                "({})",
                els.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join("+")
            ),
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
impl ToTex for Constant {
    fn to_tex(&self, _identifier: Option<&'static str>) -> String {
        match self {
            Self::Pi => "\\pi".to_string(),
        }
    }
}
impl ToTex for Operation {
    fn to_tex(&self, _: Option<&'static str>) -> String {
        match self {
            Self::Sin(s) => format!(" sin\\left({}\\right) ", s.to_tex(None)),
            Self::Cos(s) => format!(" cos\\left({}\\right) ", s.to_tex(None)),
            Self::Rem(s1, s2) => format!(" {}%{} ", s1.to_tex(None), s2.to_tex(None)),
            Self::Mul(s1, s2) => format!(
                " \\left({}\\cdot {}\\right) ",
                s1.to_tex(None),
                s2.to_tex(None)
            ),
            Self::Prod(els) => format!(
                " \\left({}\\right) ",
                els.iter()
                    .map(|e| e.to_tex(None))
                    .collect::<Vec<String>>()
                    .join(" \\cdot ")
            ),
            Self::Div(s1, s2) => format!(" \\frac{{{}}}{{{}}} ", s1.to_tex(None), s2.to_tex(None)),
            Self::Sub(s1, s2) => {
                format!(" \\left({}-{}\\right) ", s1.to_tex(None), s2.to_tex(None))
            }
            Self::Add(s1, s2) => {
                format!(" \\left({}+{}\\right) ", s1.to_tex(None), s2.to_tex(None))
            }
            Self::Sum(els) => format!(
                " \\left({}\\right) ",
                els.iter()
                    .map(|e| e.to_tex(None).to_string())
                    .collect::<Vec<String>>()
                    .join("+")
            ),
            Self::Sqrt(s) => format!(" \\sqrt{{{}}} ", s.to_tex(None)),
            Self::UnSub(s) => format!("-{}", s.to_tex(None)),
        }
    }
}

impl ToTex for Sym {
    fn to_tex(&self, _identifier: Option<&'static str>) -> String {
        match self {
            Self::Number(n) => format!("{}", n),
            // Assumes identifier to be propperly formated
            Self::Identifier(i) => i.to_string(),
            Self::Constant(c) => c.to_tex(None).to_string(),
            Self::Operation(op) => op.to_tex(None).to_string(),
        }
    }
}
