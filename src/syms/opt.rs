use matrs::predule::Matrix;
use num_traits::Zero;

use crate::{pose::Pose, syms::SignInversion};

use self::sealed::OptInner;

use super::{Sym, Operation};




mod sealed {
    use crate::syms::Sym;
    pub trait OptInner{
        fn opt_inner<'a>(&'a mut self) -> Vec<&'a Sym >;
    }
}
pub trait Opt:OptInner
where Self:Sized{
    fn opt(mut self) -> Self{
        let _ = self.opt_inner();
        self
    }
}


impl<const PREV:usize,const CURR:usize> OptInner for Pose<Sym,PREV,CURR>{
    fn opt_inner<'a>(&'a mut self) -> Vec<&'a Sym > {
        let m:&'a mut Matrix<Sym,4,4> = self.into();
        for row in 0..4{
            for col in 0..4{
                println!("Opting :{}",m[(row,col)]);
                let _ = m[(row,col)].opt_inner();
            }
        }
        vec![]
    }
}
impl<const PREV:usize,const CURR:usize> Opt for Pose<Sym,PREV,CURR>{}

impl Opt for Sym{}
impl OptInner for Sym{
    fn opt_inner<'a>(&'a mut self) -> Vec<&'a Sym> {
        println!("Opting sym : {}",self);
        
        match self{
            Sym::Number(_) => {vec![]},
            Sym::Identifier(_) => {vec![self]},
            Sym::Operation(op) => {
                op.opt_inner()
            },
            Sym::Constant(_) => vec![self]
        }
    }
}
impl Opt for Operation{}
impl OptInner for Operation{
    fn opt_inner<'a>(&'a mut self) -> Vec<&'a Sym> {
        println!("Opting op : {:?}",self); 
        let prev = self.clone();
        match self{
            Self::Add(lhs,rhs) => {
                let _ = lhs.opt_inner();
                let _ = rhs.opt_inner();
                if lhs == rhs {
                    *self = Operation::Mul(Sym::Number(2f32), lhs.clone());
                    // Early return to not cause indentation hell
                    return vec![]
                }
                if rhs.clone().sing_inversion() == *lhs {
                    *self = Self::UnSub(Sym::zero());
                    return vec![]
                }
                match (lhs.clone(),rhs.clone()) {
                    (Sym::Operation(o1), Sym::Operation(o2)) => {
                        match (*o1.clone(),*o2.clone()) {
                            (Operation::Sum(mut els),Operation::Sum(els2)) => {
                                els.extend(els2);
                                *self = Self::Sum(els);
                                let _ = self.opt_inner();
                            }
                            (Operation::Sum(mut els),e)|(e,Operation::Sum(mut els)) => {
                                els.push(Sym::Operation(Box::new(e)));
                                *self = Self::Sum(els);
                                let _ = self.opt_inner();
                            }
                            (Operation::Add(lhs_1,rhs_1),el) | (el,Operation::Add(lhs_1,rhs_1)) => {
                                let els = vec![
                                    lhs_1,rhs_1,Sym::Operation(Box::new(el))
                                ];
                                *self = Self::Sum(els);
                                let _ = self.opt_inner();
                            }
                            _ => {}


                        }
                    }
                    (el,Sym::Operation(o)) | (Sym::Operation(o),el) => {
                        match *o.clone() {
                            Operation::UnSub(s) => {
                                *self = Operation::Sub(el,s)
                            }
                            _ => {}

                        }
                    }
                    _ => {}
                }
                
                
            }
            Self::Sub(lhs,rhs) => {
                let _ = lhs.opt_inner();
                let _ = rhs.opt_inner();
                if lhs == rhs  {
                    *self = Operation::UnSub(Sym::Number(0f32));
                    return vec![];
                }
                if rhs.clone().sing_inversion() == *lhs {
                    *self = Self::UnSub(Sym::zero());
                    return vec![]
                }
                match (lhs.clone(),rhs.clone()) {
                    (el,Sym::Operation(o)) | (Sym::Operation(o),el) => {
                        match *o.clone() {
                            Operation::UnSub(s) => {
                                *self = Operation::Add(el,s)
                            }
                            _ => {}

                        }
                    }
                    _ => {}
                }
            }
            Self::Div(lhs,rhs) => {
                let _ = lhs.opt_inner();
                let _ = rhs.opt_inner();
            }
            Self::UnSub(s) => {
                let _ = s.opt_inner();
            }
            Self::Mul(lhs,rhs) => {
                let _ = lhs.opt_inner();
                let _ = rhs.opt_inner();
                match (lhs.clone(),rhs.clone()) {
                    (el,Sym::Operation(o))|(Sym::Operation(o),el) => {
                        if let Operation::UnSub(s) = *o {
                            *self = Operation::UnSub(Sym::Operation(Box::new(Self::Mul(el,s))));
                            let _ = self.opt_inner();
                            return vec![]
                        }
                    }
                    _ => {}
                }
                println!("mul these two {:?},{:?}",lhs,rhs);
                match (lhs,rhs)  {
                    (Sym::Operation(lhs),Sym::Operation(rhs)) => {
                        println!("both was mul");
                        match (*lhs.clone(),*rhs.clone()) {
                            (Self::Mul(lhs_l, lhs_r),Self::Mul(rhs_l, rhs_r)) => {
                                println!("Squashing");
                                *self = Self::Prod(vec![lhs_l,lhs_r,rhs_l,rhs_r])
                            }
                            (s,Self::Mul(rhs_l, rhs_r)) => {
                                println!("Squashing");
                                *self = Self::Prod(vec![Sym::Operation(Box::new(s)),rhs_l,rhs_r])

                            }
                            (Self::Mul(lhs_l, lhs_r),s) => {
                                println!("Squashing");
                                *self = Self::Prod(vec![lhs_l,lhs_r,Sym::Operation(Box::new(s))])
                            }
                            (Self::Prod(mut sl),Self::Prod(sr)) => {
                                sl.extend(sr);
                                *self = Self::Prod(sl);
                            }
                            (Self::Prod(mut s),r) => {
                                s.push(Sym::Operation(Box::new(r)));
                                *self = Self::Prod(s);
                            }
                            (l,Self::Prod(mut s)) => {
                                s.insert(0,Sym::Operation(Box::new(l)));
                                *self = Self::Prod(s);
                            }

                            _ => {}
                        }
                    }
                    (Sym::Operation(lhs_o),s) => {
                        println!("LHS was op : {:?}",lhs_o);
                        match *lhs_o.clone() {
                            Self::Mul(lhs_l, lhs_r)=> {
                                println!("Squashing");
                                *self = Self::Prod(vec![lhs_l,lhs_r,s.clone()]);
                            }
                            Self::Prod(mut els) => {
                                els.push(s.clone());
                                *self = Self::Prod(els.clone())
                            }
                            _ => {}
                        }

                    }
                    (s,Sym::Operation(rhs_o)) => {
                        println!("LHS was op : {:?}",rhs_o);
                        match *rhs_o.clone() {
                            Self::Mul(rhs_l, rhs_r)=> {
                                println!("Squashing");
                                *self = Self::Prod(vec![s.clone(),rhs_l,rhs_r]);
                            }
                            Self::Prod(mut els) => {
                                els.insert(0,s.clone());
                                *self = Self::Prod(els.clone())
                            }
                            _ => {}
                        }
                    }
                    _ => {} 
                }
            }
            Self::Prod(els) => {
                for el in els{
                    let _ = el.opt_inner();
                }
            }
            Self::Rem(_lhs,_rhs) => {}
            Self::Cos(s) => {
                let _ = s.opt_inner();
            }
            Self::Sin(s) => {
                let _ = s.opt_inner();
            }
            Self::Sqrt(s) => {
                let _ = s.opt_inner();
            } 
            Self::Sum(els) => {
                for s in els {
                    let _ = s.opt_inner();
                }
            }


        }
        println!("before opt : {}",prev);
        println!("after opt : {}",self);
        vec![]

    }
}


