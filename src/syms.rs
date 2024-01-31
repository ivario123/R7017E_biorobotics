
pub mod opt;
pub mod display;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
pub use display::*;
pub use opt::*;
use matrs::{matrix::rotations::Trig, CompliantNumerical};
use std::{boxed, f32::consts::PI};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Operation {
    Sqrt(Sym),
    Add(Sym, Sym),
    Sum(Vec<Sym>),
    Sub(Sym, Sym),
    UnSub(Sym),
    Div(Sym, Sym),
    Mul(Sym, Sym),
    Prod(Vec<Sym>),
    Rem(Sym, Sym),
    Cos(Sym),
    Sin(Sym),
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Constant {
    Pi,
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Sym {
    Number(f32),
    Identifier(&'static str),
    Operation(Box<Operation>),
    Constant(Constant),
}

pub trait SignInversion {
    fn sing_inversion(self) -> Self;
    fn negative(&self) -> bool;
}

impl SignInversion for Sym {
    fn sing_inversion(self) -> Self {
        match self.clone() {
            Self::Number(n) => Self::Number(-n),
            Self::Operation(o) => match *o {
                Operation::UnSub(s) => s,
                _ => Self::Operation(Box::new(Operation::UnSub(self))),
            },
            e => Self::Operation(Box::new(Operation::UnSub(e))),
        }
    }
    fn negative(&self) -> bool {
        match self {
            Self::Number(n) => *n < 0f32,
            Self::Operation(o) => match **o {
                Operation::UnSub(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl CompliantNumerical for Sym {
    fn sqrt(num: Self) -> Self {
        Sym::Operation(Box::new(Operation::Sqrt(num)))
    }
}

impl Default for Sym {
    fn default() -> Self {
        Self::Number(0f32)
    }
}

impl Trig for Sym {
    fn pi() -> Self {
        Sym::Constant(Constant::Pi)
    }
    fn sine(self) -> Self {
        match self.clone() {
            Self::Operation(op) => match *op {
                Operation::Cos(el) => return el,
                _ => {}
            },
            // Exact values
            Self::Number(n) => {
                return Self::Number(n.sin())

            },
            _ => {}
        }
        Self::Operation(Box::new(Operation::Sin(self)))
    }
    fn cosine(self) -> Self {
        match self.clone() {
            Self::Operation(op) => match *op {
                Operation::Sin(el) => return el,
                _ => {}
            },
            // Exact values
            Self::Number(n) => {
                return Self::Number(n.cos())
                            },
            _ => {}
        }
        Self::Operation(Box::new(Operation::Cos(self)))
    }
}

impl Add<f32> for Sym {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        if let Sym::Number(n) = &self{
            return Self::Number(n+rhs);
        }
        self + Sym::Number(rhs)
    }
}
impl Sub<f32> for Sym {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        if let Sym::Number(n) = &self{
            return Self::Number(n-rhs);
        }
        self - Sym::Number(rhs)
    }
}

impl Div<f32> for Sym {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        if let Sym::Number(n) = &self{
            return Self::Number(n/rhs);
        }
        self / Sym::Number(rhs)
    }
}

impl Mul<f32> for Sym {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        if let Sym::Number(n) = &self{
            return Self::Number(n*rhs);
        }
        self * Sym::Number(rhs)
    }
}

impl Rem<f32> for Sym {
    type Output = Self;
    fn rem(self, rhs: f32) -> Self::Output {
        self % Sym::Number(rhs)
    }
}

impl Add for Sym {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self == Self::Number(0f32) {
            return rhs;
        }
        if rhs == Self::Number(0f32) {
            return self;
        }
        match (&self,&rhs) {
            (Sym::Number(n1),Sym::Number(n2)) =>return Sym::Number(n1+n2),
            _ => {}
        }


        Sym::Operation(Box::new(Operation::Add(self, rhs)))
    }
}
impl Sub for Sym {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if rhs == Self::Number(0f32) {
            return self;
        }
        if self == Self::Number(0f32) {
            return Self::Operation(Box::new(Operation::UnSub(rhs)));
        }
        match (&self,&rhs) {
            (Sym::Number(n1),Sym::Number(n2)) =>return Sym::Number(n1-n2),
            _ => {}
        }


        Sym::Operation(Box::new(Operation::Sub(self, rhs)))
    }
}

impl Div for Sym {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut lhs = self;
        let mut rhs = rhs;
        if lhs == Self::Number(0f32) {
            return Self::Number(0f32);
        }
        if rhs == Self::Number(1f32) {
            return lhs;
        }
        if let Self::Number(n) = rhs.clone() {
            if n < 0f32 {
                rhs = rhs.sing_inversion();
                lhs = lhs.sing_inversion();
            }
        }
        match (&lhs,&rhs) {
            (Sym::Number(n1),Sym::Number(n2)) =>return Sym::Number(n1/n2),
            _ => {}
        }


        Sym::Operation(Box::new(Operation::Div(lhs, rhs)))
    }
}

impl Mul for Sym {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut lhs = self;
        let mut rhs = rhs;
        if lhs == Self::Number(1f32) {
            return rhs;
        }
        if rhs == Self::Number(1f32) {
            return lhs;
        }
        if rhs.negative() || lhs.negative() {
            rhs = rhs.sing_inversion();
            lhs = lhs.sing_inversion();
        }
        if lhs == Self::Number(0f32) {
            return Self::Number(0f32);
        }
        if rhs == Self::Number(0f32) {
            return Self::Number(0f32);
        }
        if lhs == Self::Number(1f32) {
            return rhs;
        }
        if rhs == Self::Number(1f32) {
            return lhs;
        }
        match (&lhs,&rhs) {
            (Sym::Number(n1),Sym::Number(n2)) =>return Sym::Number(n1*n2),
            _ => {}
        }

        Sym::Operation(Box::new(Operation::Mul(lhs, rhs)))
    }
}

impl Rem for Sym {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Sym::Operation(Box::new(Operation::Mul(self, rhs)))
    }
}

impl DivAssign for Sym {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs
    }
}

impl MulAssign for Sym {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl AddAssign for Sym {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}
impl SubAssign for Sym {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl RemAssign for Sym {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.clone() % rhs;
    }
}
impl num_traits::Num for Sym {
    type FromStrRadixErr = ();
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}
impl num_traits::One for Sym {
    fn one() -> Self {
        Self::Number(1f32)
    }
}
impl num_traits::Zero for Sym {
    fn is_zero(&self) -> bool {
        match self {
            Self::Number(e) => *e == 0f32,
            _ => todo!(),
        }
    }
    fn set_zero(&mut self) {
        *self = Self::Number(1f32)
    }
    fn zero() -> Self {
        Self::Number(0f32)
    }
}

#[macro_export]
macro_rules! sym {
    ($id:literal) => {
        Sym::Identifier($id)
    };
}
