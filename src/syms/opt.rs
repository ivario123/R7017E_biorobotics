use array_init::array_init;
use matrs::{predule::Matrix, vec};
use num_traits::Zero;

use crate::{
    pose::Pose,
    syms::{Constant, SignInversion},
};

use self::sealed::OptInner;

use super::{Operation, Sym};

trait Hash {
    fn hash_internal(&self) -> u8;
    fn hash(&self) -> usize {
        (self.hash_internal() & 0b1111) as usize
    }
}

impl Hash for f32 {
    fn hash_internal(&self) -> u8 {
        self.to_bits().to_le_bytes()[0]
    }
}

impl Hash for &str {
    /// This implementation is kinda slow. should probably
    fn hash_internal(&self) -> u8 {
        let n_bit = |byte, idx| byte & ((1 << idx) as u8);
        let mut collector = 0;
        for (idx, el) in self.bytes().into_iter().enumerate() {
            if idx == 8 {
                break;
            }
            collector |= n_bit(el, idx);
        }
        return collector;
    }
}
#[derive(Debug)]
struct HashSet {
    elements: usize,
    fields: [Vec<Sym>; (1 << 4) - 1],
    hashes: Vec<Sym>,
}
impl HashSet {
    fn new() -> Self {
        Self {
            elements: 0,
            fields: array_init(|_| Vec::new()),
            hashes: Vec::new(),
        }
    }

    fn insert(&mut self, el: Sym) -> bool {
        let hash = match el.clone() {
            Sym::Number(1f32) => return false,
            Sym::Number(n) => n.hash(),
            Sym::Constant(c) => c.to_string().as_str().hash(),
            Sym::Identifier(i) => i.hash(),
            Sym::Operation(o) => o.to_string().as_str().hash(),
        };
        for curr_el in self.fields[hash].iter() {
            if *curr_el == el {
                return true;
            }
        }
        self.fields[hash].push(el.clone());
        self.hashes.push(el);
        self.elements += 1;
        return true;
    }
    fn contains(&self, el: &Sym) -> bool {
        let hash = match el {
            Sym::Number(n) => n.hash(),
            Sym::Constant(c) => c.to_string().as_str().hash(),
            Sym::Identifier(i) => i.hash(),
            Sym::Operation(_) => return false,
        };
        for curr_el in self.fields[hash].iter() {
            if *curr_el == *el {
                return true;
            }
        }
        return false;
    }
    fn combine(&self, other: &Self) -> HashSet {
        let mut ret = HashSet::new();
        for el in self.hashes.iter() {
            ret.insert(el.clone());
        }
        for el in other.hashes.iter() {
            ret.insert(el.clone());
        }
        ret
    }
    fn union(&self, other: &Self) -> HashSet {
        let (subset, superset) = match self.elements < other.elements {
            true => (self, other),
            _ => (other, self),
        };
        let mut ret = HashSet::new();

        for el in subset.hashes.iter() {
            if superset.contains(&el) {
                ret.insert(el.clone());
            }
        }
        ret
    }
    fn subtract(&self, other: &Self) -> Self {
        let mut ret = HashSet::new();
        let (subset, superset) = match self.elements < other.elements {
            true => (self, other),
            _ => return ret,
        };

        for el in subset.hashes.iter() {
            if !superset.contains(el) {
                ret.insert(el.clone());
            }
        }
        ret
    }
}

mod sealed {
    use super::HashSet;
    use crate::syms::Sym;
    pub trait OptInner {
        fn opt_inner(&mut self) -> Vec<&Sym>;
        fn longest_common(&self) -> HashSet;
        fn replace_common(&mut self, set: &HashSet);
    }
}
pub trait Opt: OptInner
where
    Self: Sized,
{
    fn opt(mut self) -> Self {
        let _ = self.opt_inner();
        self
    }
}

impl Sym {
    fn eq_internal(&self, other: &Self) -> bool {
        // println!("Comparing {self},{other}");
        let res = match (self, other) {
            (Sym::Number(n1), Sym::Number(n2)) => n2 == n1,
            (Sym::Constant(c1), Sym::Constant(c2)) => c1 == c2,
            (Sym::Operation(o1), Sym::Operation(o2)) => o1.eq_internal(o2),
            (Sym::Identifier(i1), Sym::Identifier(i2)) => i1 == i2,
            _ => false,
        };
        // println!("Returning {res}");
        res
    }
}

impl Operation {
    fn eq_internal(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sqrt(s1), Self::Sqrt(s2))
            | (Self::Cos(s1), Self::Cos(s2))
            | (Self::Sin(s1), Self::Sin(s2))
            | (Self::UnSub(s1), Self::UnSub(s2)) => s1.eq_internal(s2),
            (Self::Add(s1, s2), Self::Add(s3, s4))
            | (Self::Sub(s1, s2), Self::Sub(s3, s4))
            | (Self::Div(s1, s2), Self::Div(s3, s4))
            | (Self::Mul(s1, s2), Self::Mul(s3, s4)) => s1.eq_internal(s3) && s2.eq_internal(s4),
            (Self::Prod(s1), Self::Prod(s2)) | (Self::Sum(s1), Self::Sum(s2)) => {
                s1.iter().zip(s2).all(|(s1, s2)| s1.eq_internal(s2))
            }
            _ => false,
        }
    }
}

impl<const PREV: usize, const CURR: usize> OptInner for Pose<Sym, PREV, CURR> {
    fn opt_inner<'a>(&'a mut self) -> Vec<&'a Sym> {
        let m: &'a mut Matrix<Sym, 4, 4> = self.into();
        for row in 0..4 {
            for col in 0..4 {
                // Opt in to local minima
                let mut prev = m[(row, col)].clone();
                let mut res = prev.clone();
                res.opt_inner();
                while !prev.eq_internal(&res) {
                    prev = m[(row, col)].clone();
                    res = prev.clone();
                    res.opt_inner();
                    m[(row, col)] = res.clone();
                }
            }
        }
        vec![]
    }
    fn longest_common(&self) -> HashSet {
        todo!()
    }
    fn replace_common(&mut self, set: &HashSet) {
        todo!()
    }
}
impl<const PREV: usize, const CURR: usize> Opt for Pose<Sym, PREV, CURR> {}

impl Opt for Sym {}
impl OptInner for Sym {
    fn opt_inner(&mut self) -> Vec<&Sym> {
        match self.clone() {
            Sym::Number(_) => {
                vec![]
            }
            Sym::Identifier(_) => {
                vec![self]
            }

            Sym::Operation(op) => match *op.clone() {
                Operation::Nop(s) => {
                    let mut s = s.clone();
                    s.opt_inner();
                    *self = s;
                    return vec![];
                }
                _ => {
                    let mut op = op.clone();
                    op.opt_inner();
                    *self = Sym::Operation(op);
                    return vec![];
                }
            },
            Sym::Constant(_) => vec![self],
        }
    }
    fn longest_common(&self) -> HashSet {
        let mut ret = HashSet::new();
        match self {
            Self::Number(_) | Self::Identifier(_) | Self::Constant(_) => {
                ret.insert(self.clone());
            }
            Self::Operation(o) => return o.longest_common(),
        }
        ret
    }
    fn replace_common(&mut self, set: &HashSet) {
        match self {
            Self::Number(_) | Self::Identifier(_) | Self::Constant(_) => {
                // println!(
                //     "self : {self},contains : {:?},set {:?}",
                //     set,
                //     set.contains(self)
                // );
                if set.contains(self) {
                    *self = Sym::Number(1f32);
                }
            }
            Self::Operation(o) => o.replace_common(set),
        }
    }
}
impl Opt for Operation {}
fn is_atomic(op: &Operation) -> bool {
    match op {
        Operation::Cos(_) | Operation::Sqrt(_) | Operation::Sin(_) | Operation::UnSub(_) => true,
        _ => false,
    }
}
fn is_zero(op: &Operation) -> bool {
    match op {
        Operation::UnSub(s) | Operation::Nop(s) => match s {
            Sym::Number(0f32) => true,
            Sym::Operation(o) => is_zero(o),
            _ => false,
        },
        Operation::Sum(s) => s.iter().all(|el| is_zero_sym(el)),
        Operation::Prod(s) => s.iter().any(|el| is_zero_sym(el)),
        _ => false,
    }
}
fn is_zero_sym(op: &Sym) -> bool {
    match op {
        Sym::Number(0f32) => true,
        Sym::Operation(o) => is_zero(o),
        _ => false,
    }
}

fn prio(s: &Sym) -> usize {
    match s {
        Sym::Identifier(_) => 0,
        Sym::Number(_) => 1,
        Sym::Constant(_) => 2,
        Sym::Operation(_) => 3,
    }
}
fn sort_add(op: &mut Operation){
    let (rhs,lhs) = match op {
        Operation::Add(s1,s2) => (s1,s2),
        _ => {return;}
    };
    let lhsp = prio(lhs);
    let rhsp = prio(rhs);
    let (rhs,lhs) = match rhsp>=lhsp{
        true => (rhs,lhs),
        false => (lhs,rhs)
    };
    *op = Operation::Mul(rhs.clone(), lhs.clone());
}
fn sort_mul(op: &mut Operation){
    let (rhs,lhs) = match op {
        Operation::Mul(s1,s2) => (s1,s2),
        _ => {return;}
    };
    let lhsp = prio(lhs);
    let rhsp = prio(rhs);
    let (rhs,lhs) = match (rhsp>lhsp,rhsp==lhsp){
        (true,_) => (rhs,lhs),
        (false,_) => (lhs,rhs),
        (_,_)   => {
            match rhs.to_string()>lhs.to_string(){
                true => (rhs,lhs),
                _ => (rhs,lhs)
            }
        }
    };
    *op = Operation::Mul(rhs.clone(), lhs.clone());
}
fn sort(op: &mut Operation) {
    let sum = match op {
        Operation::Sum(_s) => true,
        Operation::Prod(_s) => false,
        Operation::Mul(_,_) => { 
            sort_mul(op);
            return;
        }
        Operation::Add(_,_) => { 
            sort_add(op);
            return;
        }

        _ => return,
    };
    let l = match op {
        Operation::Sum(s) |
        Operation::Prod(s) => s,
        _ => return,
    };
    let mut l = l
        .iter_mut()
        .map(|el| (prio(el), el))
        .collect::<Vec<(usize, &mut Sym)>>();
    l.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.to_string().cmp(&b.1.to_string())
        } else {
            a.0.cmp(&b.0)
        }
    });
    let l = l.into_iter().map(|e| e.1.clone()).collect();
    if sum {
        *op = Operation::Sum(l);
        return;
    }
    *op = Operation::Prod(l)
}

fn to_prod(op: &mut Operation) -> bool {
    match op {
        Operation::Mul(Sym::Operation(o1), Sym::Operation(o2)) => {
            match (*o1.clone(), *o2.clone()) {
                (Operation::Mul(s1, s2), Operation::Mul(s3, s4)) => {
                    *op = Operation::Prod(vec![s1, s2, s3, s4]);
                    to_prod(op);
                }
                (Operation::Mul(s1, s2), Operation::Prod(mut p))
                | (Operation::Prod(mut p), Operation::Mul(s1, s2)) => {
                    p.push(s1);
                    p.push(s2);
                    *op = Operation::Prod(p.clone());
                    to_prod(op);
                }
                _ => {
                    return false;
                }
            }
        }
        Operation::Prod(p) => {
            let mut ret = vec![];
            for el in p.iter() {
                match el.clone() {
                    Sym::Operation(o) => {
                        let mut intermediate = *o.clone();
                        to_prod(&mut intermediate);
                        match intermediate {
                            Operation::Prod(p) => {
                                ret.extend(p.clone());
                            }
                            _ => ret.push(el.clone()),
                        };
                    }
                    _ => ret.push(el.clone()),
                }
            }
            *op = Operation::Prod(ret);
        }
        _ => return false,
    }
    sort(op);
    true
}

fn opt_trig(op: &mut Operation) -> bool {
    // Finds common trig identities
    let sign = match op {
        Operation::Add(_, _) => false,
        Operation::Sub(_, _) => true,
        _ => {
            return false;
        }
    };
    match op.clone() {
        Operation::Add(s1, s2) | Operation::Sub(s1, s2) => {
            // Lets start with simple additive formulas
            let (o1, o2) = match (s1, s2) {
                (Sym::Operation(o1), Sym::Operation(o2)) => (o1, o2),
                _ => {
                    return false;
                }
            };
            let (s1, s2, s3, s4) = match (*o1.clone(), *o2.clone()) {
                (Operation::Mul(s1, s2), Operation::Mul(s3, s4)) => (s1, s2, s3, s4),
                (Operation::Prod(s1), Operation::Prod(s2)) => {
                    let (l1, l2) = (s1.len(), s2.len());
                    if l1 != l2 || l1 != 2 || l2 != 2 {
                        return false;
                    }
                    (s1[0].clone(), s1[1].clone(), s2[0].clone(), s2[1].clone())
                }
                _ => {
                    return false;
                }
            };
            let (o1, o2, o3, o4) = match (s1, s2, s3, s4) {
                (
                    Sym::Operation(o1),
                    Sym::Operation(o2),
                    Sym::Operation(o3),
                    Sym::Operation(o4),
                ) => (o1, o2, o3, o4),
                _ => {
                    return false;
                }
            };
            let mut sin = true;
            let mut cos_first = false;
            let (s1, s2) = match (*o1.clone(), *o2.clone()) {
                (Operation::Sin(s1), Operation::Cos(s2)) => (s1, s2),
                (Operation::Cos(s2), Operation::Sin(s1)) => (s1, s2),
                (Operation::Cos(s1), Operation::Cos(s2)) => {
                    cos_first = true;
                    sin = false;
                    (s1, s2)
                }
                (Operation::Sin(s1), Operation::Sin(s2)) => {
                    sin = false;
                    (s1, s2)
                }
                _ => {
                    return false;
                }
            };
            let (s3, s4) = match (*o3.clone(), *o4.clone(), cos_first) {
                (Operation::Sin(s2), Operation::Cos(s1), _) => (s1, s2),
                (Operation::Cos(s1), Operation::Sin(s2), _) => (s1, s2),
                (Operation::Sin(s1), Operation::Sin(s2), true) => {
                    sin = false;
                    (s1, s2)
                }
                (Operation::Cos(s1), Operation::Cos(s2), false) => {
                    sin = false;
                    (s1, s2)
                }
                _ => {
                    return false;
                }
            };
            if s1 == s3 && s2 == s4 {
                match sin {
                    true => {
                        *op = Operation::Sin(Sym::Operation(Box::new(if !sign {
                            Operation::Add(s1, s2)
                        } else {
                            Operation::Sub(s1, s2)
                        })));
                    }
                    false => {
                        *op = Operation::Cos(Sym::Operation(Box::new(if !sign {
                            Operation::Sub(s1, s2)
                        } else {
                            Operation::Add(s1, s2)
                        })));
                    }
                }
                return true;
            }
            if !sin && s1 == s4 && s2 == s3 {
                *op = Operation::Cos(Sym::Operation(Box::new(if !sign {
                    Operation::Sub(s1, s2)
                } else {
                    Operation::Add(s1, s2)
                })));
                return true;
            }
        }
        _ => {}
    }
    false
}

fn is_unsub(op: &Sym) -> Option<Sym> {
    let o = match op {
        Sym::Operation(o) => o.clone(),
        _ => {
            return None;
        }
    };
    match *o {
        Operation::UnSub(s) => Some(s.clone()),
        _ => None,
    }
}
impl OptInner for Operation {
    fn opt_inner(&mut self) -> Vec<&Sym> {
        if opt_trig(self) {
            self.opt_inner();
            return vec![];
        }

        let _ = to_prod(self);
        match self {
            Self::Add(lhs, rhs) => {
                let _ = lhs.opt_inner();
                let _ = rhs.opt_inner();
                if let Some(s) = is_unsub(lhs) {
                    *self = Operation::Sub(rhs.clone(), s);
                    return vec![];
                }
                if let Some(s) = is_unsub(rhs) {
                    *self = Operation::Sub(lhs.clone(), s);
                    return vec![];
                }
                if is_zero_sym(lhs) {
                    *self = Self::Nop(rhs.clone());
                    return vec![];
                }
                if is_zero_sym(rhs) {
                    *self = Self::Nop(lhs.clone());
                    return vec![];
                }
                if lhs == rhs {
                    *self = Operation::Mul(Sym::Number(2f32), lhs.clone());
                    // Early return to not cause indentation hell
                    return vec![];
                }
                if rhs.clone().sing_inversion() == *lhs {
                    *self = Self::UnSub(Sym::zero());
                    return vec![];
                }
                match (lhs.clone(), rhs.clone()) {
                    (Sym::Operation(o1), Sym::Operation(o2)) => match (*o1.clone(), *o2.clone()) {
                        (Operation::Sum(mut els), Operation::Sum(els2)) => {
                            els.extend(els2);
                            *self = Self::Sum(els);
                        }
                        (Operation::Sum(mut els), e) | (e, Operation::Sum(mut els)) => {
                            els.push(Sym::Operation(Box::new(e)));
                            *self = Self::Sum(els);
                        }
                        (Operation::Add(lhs_1, rhs_1), el) | (el, Operation::Add(lhs_1, rhs_1)) => {
                            let els = vec![lhs_1, rhs_1, Sym::Operation(Box::new(el))];
                            *self = Self::Sum(els);
                        }
                        _ => {}
                    },
                    (el, Sym::Operation(o)) | (Sym::Operation(o), el) => {
                        if let Operation::UnSub(s) = *o.clone() {
                            *self = Operation::Sub(el, s)
                        }
                    }
                    _ => {}
                }
            }
            Self::Sub(lhs, rhs) => {
                let _ = lhs.opt_inner();
                let _ = rhs.opt_inner();
                if let Sym::Number(0f32) = lhs {
                    *self = Operation::Nop(rhs.clone());
                    return vec![];
                }
                if let Sym::Number(0f32) = rhs {
                    *self = Operation::UnSub(lhs.clone());
                    return vec![];
                }

                if lhs == rhs {
                    *self = Operation::UnSub(Sym::Number(0f32));
                    return vec![];
                }
                if rhs.clone().sing_inversion() == *lhs {
                    *self = Self::UnSub(Sym::zero());
                    return vec![];
                }
                match (lhs.clone(), rhs.clone()) {
                    (el, Sym::Operation(o)) | (Sym::Operation(o), el) => {
                        if let Operation::UnSub(s) = *o.clone() {
                            *self = Operation::Add(el, s)
                        }
                    }
                    _ => {}
                }
            }
            Self::Div(lhs, rhs) => {
                let _ = lhs.opt_inner();
                let _ = rhs.opt_inner();
                if let Sym::Number(1f32) = rhs {
                    *self = Self::Prod(vec![lhs.clone()]);
                    return vec![];
                }
                if let Sym::Number(-1f32) = rhs {
                    *self = Self::UnSub(lhs.clone());
                    return vec![];
                }
                if lhs == rhs {
                    *self = Self::UnSub(Sym::zero());
                    return vec![];
                }
                let set = lhs.longest_common().union(&rhs.longest_common());
                lhs.replace_common(&set);
                rhs.replace_common(&set);
                *self = Self::Div(lhs.clone(), rhs.clone());
            }
            Self::UnSub(Sym::Operation(o)) => match *o.clone() {
                Operation::UnSub(s) => *self = Self::Nop(s),
                _ => {
                    o.opt_inner();
                }
            },
            Self::UnSub(s) => {
                let _ = s.opt_inner();
            }
            Self::Mul(lhs, rhs) => {
                let _ = lhs.opt_inner();
                let _ = rhs.opt_inner();
                if let Some(s) = is_unsub(lhs) {
                    *self = Self::Mul(rhs.clone(), s);
                    return vec![];
                }
                if let Some(s) = is_unsub(rhs) {
                    *self = Self::Mul(lhs.clone(), s);
                    return vec![];
                }

                if is_zero_sym(lhs) {
                    *self = Self::Nop(Sym::Number(0f32));
                    return vec![];
                }
                if is_zero_sym(rhs) {
                    *self = Self::Nop(Sym::Number(0f32));
                    return vec![];
                }
                match (lhs.clone(), rhs.clone()) {
                    (el, Sym::Operation(o)) | (Sym::Operation(o), el) => {
                        if let Operation::UnSub(s) = *o {
                            *self = Operation::UnSub(Sym::Operation(Box::new(Self::Mul(el, s))));
                            //let _ = self.opt_inner();
                            return vec![];
                        }
                    }
                    _ => {}
                }
                match (lhs, rhs) {
                    (Sym::Operation(lhs), Sym::Operation(rhs)) => {
                        match (*lhs.clone(), *rhs.clone()) {
                            (Self::Mul(lhs_l, lhs_r), Self::Mul(rhs_l, rhs_r)) => {
                                *self = Self::Prod(vec![lhs_l, lhs_r, rhs_l, rhs_r])
                            }
                            (s, Self::Mul(rhs_l, rhs_r)) => {
                                *self = Self::Prod(vec![Sym::Operation(Box::new(s)), rhs_l, rhs_r])
                            }
                            (Self::Mul(lhs_l, lhs_r), s) => {
                                *self = Self::Prod(vec![lhs_l, lhs_r, Sym::Operation(Box::new(s))])
                            }
                            (Self::Prod(mut sl), Self::Prod(sr)) => {
                                sl.extend(sr);
                                *self = Self::Prod(sl);
                            }
                            (Self::Prod(mut s), r) => {
                                s.push(Sym::Operation(Box::new(r)));
                                *self = Self::Prod(s);
                            }
                            (l, Self::Prod(mut s)) => {
                                s.insert(0, Sym::Operation(Box::new(l)));
                                *self = Self::Prod(s);
                            }

                            _ => {}
                        }
                    }
                    (Sym::Operation(lhs_o), s) => {
                        if let Sym::Number(1f32) = s {
                            *self = Self::Prod(vec![Sym::Operation(lhs_o.clone())]);
                            return vec![];
                        }
                        match *lhs_o.clone() {
                            Self::Mul(lhs_l, lhs_r) => {
                                *self = Self::Prod(vec![lhs_l, lhs_r, s.clone()]);
                            }
                            Self::Prod(mut els) => {
                                els.push(s.clone());
                                *self = Self::Prod(els.clone())
                            }
                            _ => {}
                        }
                    }
                    (s, Sym::Operation(rhs_o)) => match *rhs_o.clone() {
                        Self::Mul(rhs_l, rhs_r) => {
                            *self = Self::Prod(vec![s.clone(), rhs_l, rhs_r]);
                        }
                        Self::Prod(mut els) => {
                            els.insert(0, s.clone());
                            *self = Self::Prod(els.clone())
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            Self::Prod(els) => {
                let mut els = els.clone();
                let mut keep = vec![];
                let mut negative = false;
                for el in els.iter_mut() {
                    if let Sym::Number(1f32) = el {
                        continue;
                    }
                    if let Sym::Number(-1f32) = el {
                        negative = !negative;
                        continue;
                    }
                    if let Sym::Operation(o) = el.clone() {
                        if let Operation::UnSub(s) = *o {
                            negative = !negative;
                            if let Sym::Number(1f32) = s {
                                continue;
                            }
                            if let Sym::Number(0f32) = s {
                                *self = Self::Prod(vec![Sym::Number(0f32)]);
                                return vec![];
                            }
                            *el = s.clone();
                        }
                    }
                    if is_zero_sym(el) {
                        *self = Self::Prod(vec![Sym::Number(0f32)]);
                        return vec![];
                    }
                    el.opt_inner();
                    keep.push(el.clone());
                }

                *self = match negative {
                    false => Self::Prod(keep),
                    true => Self::UnSub(Sym::Operation(Box::new(Self::Prod(keep)))),
                }
            }
            Self::Rem(_lhs, _rhs) => {
                todo!()
            }
            Self::Cos(s) => {
                let mut s = s.clone();
                let o = match s.clone() {
                    Sym::Operation(o) => *o.clone(),
                    Sym::Constant(Constant::Pi) => {
                        *self = Self::Cos(Sym::Number(-1f32));
                        return vec![];
                    }
                    Sym::Number(0f32) => {
                        *self = Self::Cos(Sym::Number(-1f32));
                        return vec![];
                    }
                    _ => {
                        s.opt_inner();
                        return vec![];
                    }
                };
                if let Operation::Div(Sym::Constant(Constant::Pi), Sym::Number(2f32)) = o {
                    *self = Operation::UnSub(Sym::Number(0f32));
                }
                if let Operation::Div(mut s1, mut s2) = o {
                    let o = match s1 {
                        Sym::Operation(o) => *o.clone(),
                        _ => {
                            s.opt_inner();
                            return vec![];
                        }
                    };
                    if let Operation::UnSub(Sym::Constant(Constant::Pi)) = o {
                        if let Sym::Number(2f32) = s2 {
                            *self = Operation::UnSub(Sym::Number(0f32));
                            return vec![];
                        }
                    }
                }
                // TODO! opt out n*PI
                let _ = s.opt_inner();
            }
            Self::Sin(s) => {
                let mut s = s.clone();
                let o = match s.clone() {
                    Sym::Operation(o) => *o.clone(),
                    Sym::Constant(Constant::Pi) => {
                        *self = Self::Cos(Sym::Number(0f32));
                        return vec![];
                    }
                    Sym::Number(0f32) => {
                        *self = Self::Cos(Sym::Number(0f32));
                        return vec![];
                    }
                    _ => {
                        s.opt_inner();
                        return vec![];
                    }
                };
                if let Operation::Div(Sym::Constant(Constant::Pi), Sym::Number(2f32)) = o {
                    *self = Operation::UnSub(Sym::Number(1f32));
                }
                if let Operation::Div(mut s1, mut s2) = o {
                    let o = match s1 {
                        Sym::Operation(o) => *o.clone(),
                        _ => {
                            s.opt_inner();
                            return vec![];
                        }
                    };
                    if let Operation::UnSub(Sym::Constant(Constant::Pi)) = o {
                        if let Sym::Number(2f32) = s2 {
                            *self = Operation::UnSub(Sym::Number(-1f32));
                            return vec![];
                        }
                    }
                }
                // TODO! opt out n*PI/2
                let _ = s.opt_inner();
            }
            Self::Sqrt(s) => {
                let _ = s.opt_inner();
            }
            Self::Sum(els) => {
                let mut els = els.clone();
                let mut keep = vec![];
                for el in els.iter_mut() {
                    if is_zero_sym(el) {
                        continue;
                    }
                    if let Sym::Operation(o) = el {
                        if let Operation::Nop(Sym::Number(0f32)) = **o {
                            continue;
                        }
                    }
                    el.opt_inner();
                    keep.push(el.clone());
                }
                *self = Self::Prod(keep);
            }
            Self::Nop(s) => {
                if let Sym::Operation(o) = s.clone() {
                    println!("{o:?}");
                    if let Operation::Nop(s) = *o.clone() {
                        *self = Operation::Nop(s.clone());
                        self.opt_inner();
                        return vec![];
                    }
                }

                s.opt_inner();
            }
        }

        if let Self::Prod(_els) = self {
            return vec![];
        }
        // // Ensure that we do not iterate for infinity
        // if let Self::Mul(_, _) = self {
        //     return vec![];
        // }
        let cmn = self.longest_common();
        if cmn.elements != 0 && !is_atomic(self) {
            let mut lhs = self.clone();
            lhs.replace_common(&cmn);
            let multiplier = Sym::Operation(Box::new(Self::Prod(cmn.hashes)));
            *self = Self::Mul(multiplier, Sym::Operation(Box::new(lhs)));
            self.opt_inner();
            sort(self);
        }
        vec![]
    }
    fn longest_common(&self) -> HashSet {
        let ret = match self {
            Self::Div(s1, s2) => s1.longest_common().subtract(&s2.longest_common()),
            Self::Add(s1, s2) | Self::Sub(s1, s2) => {
                s1.longest_common().union(&s2.longest_common())
            }
            Self::Mul(s1, s2) => s1.longest_common().combine(&s2.longest_common()),
            Self::Prod(syms) => {
                let mut ret = HashSet::new();
                syms.clone()
                    .into_iter()
                    .for_each(|s| ret = ret.combine(&s.longest_common()));
                ret
            }
            Self::Sum(syms) => {
                let mut ret = syms[0].clone().longest_common();
                syms.iter()
                    .for_each(|s| ret = ret.union(&s.longest_common()));
                ret
            }
            // These are atomic
            Self::UnSub(_s) | Self::Cos(_s) | Self::Sin(_s) | Self::Sqrt(_s) => {
                let mut ret = HashSet::new();
                ret.insert(Sym::Operation(Box::new(self.clone())));
                ret
            }
            Self::Rem(_, _) => todo!(),
            Self::Nop(s) => s.longest_common(),
        };
        ret
    }
    fn replace_common(&mut self, set: &HashSet) {
        match self {
            Self::Add(s1, s2) => {
                s1.replace_common(set);
                s2.replace_common(set);
                *self = Self::Add(s1.clone(), s2.clone());
            }
            Self::Sub(s1, s2) => {
                s1.replace_common(set);
                s2.replace_common(set);
                *self = Self::Sub(s1.clone(), s2.clone());
            }
            Self::Mul(s1, s2) => {
                s1.replace_common(set);
                s2.replace_common(set);
                *self = Self::Mul(s1.clone(), s2.clone());
            }
            Self::Div(s1, s2) => {
                s1.replace_common(set);
                s2.replace_common(set);
                *self = Self::Div(s1.clone(), s2.clone());
            }
            Self::Rem(s1, s2) => {
                s1.replace_common(set);
                s2.replace_common(set);
                *self = Self::Rem(s1.clone(), s2.clone());
            }
            Self::Prod(syms) => {
                for el in syms.iter_mut() {
                    el.replace_common(set);
                }
                *self = Self::Prod(syms.clone());
            }
            Self::Sum(syms) => {
                for el in syms.iter_mut() {
                    el.replace_common(set);
                }
                *self = Self::Sum(syms.clone());
            }
            Self::UnSub(_s) | Self::Cos(_s) | Self::Sin(_s) | Self::Sqrt(_s) => {
                if set.contains(&Sym::Operation(Box::new(self.clone()))) {
                    *self = Self::UnSub(Sym::zero());
                }
            }
            Self::Nop(s) => s.replace_common(set),
        }

        // println!("Replaced all {:?} in {self:?}", set);
    }
}
