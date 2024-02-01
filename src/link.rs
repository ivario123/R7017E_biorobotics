use matrs::{
    matrix::helpers::rotations::*, matrix::rotations::Trig, predule::*, CompliantNumerical,
};

use std::ops::Mul;

use crate::decore::decorators::ToTex;
use crate::pose::Pose;

// pub struct Link<const IDX: usize,L:CompliantNumerical+Trig,> {
//      length:
// }

pub struct DHTable {
    rows: Vec<[String; 4]>,
}

impl DHTable {
    fn new<
        Theta: CompliantNumerical + Trig + ToTex,
        D: CompliantNumerical + Trig + ToTex,
        A: CompliantNumerical + Trig + ToTex,
        Alpha: CompliantNumerical + Trig + ToTex,
    >(
        theta: &Theta,
        d: &D,
        a: &A,
        alpha: &Alpha,
    ) -> Self {
        Self {
            rows: vec![[
                theta.to_tex(None),
                d.to_tex(None),
                a.to_tex(None),
                alpha.to_tex(None),
            ]],
        }
    }
    pub fn extend(mut self, other: Self) -> Self {
        self.rows.extend(other.rows);
        self
    }
    pub fn to_tex(&self) -> String {
        let mut ret:String = "\\begin{table}[H]\\label{table:DHParams}\\centering\n\t\\begin{tabular}{|c|c|c|c|c|}\\hline\n".to_string();

        ret += "\t\t$j$\t&\t$\\theta$\t&\t$d$\t&\t$a$\t&\t$\\alpha$\t\\\\\\hline\n";
        for (idx, row) in self.rows.iter().enumerate() {
            ret += format!(
                "\t\t${}$\t&\t${}$\t&\t${}$\t&\t${}$\t&\t${}$\\\\\n",
                idx + 1,
                row[0],
                row[1],
                row[2],
                row[3]
            )
            .as_str();
        }
        ret += "\t\\hline\n\t\\end{tabular}\n\t\\caption{good caption here}\n\\end{table}";

        ret
    }
}

pub struct DHBuilder<
    const THETA: bool,
    const D: bool,
    const A: bool,
    const ALPHA: bool,
    Thetat: CompliantNumerical + Trig,
    Dt: CompliantNumerical + Trig,
    At: CompliantNumerical + Trig,
    Alphat: CompliantNumerical + Trig,
> {
    theta: Option<Thetat>,
    d: Option<Dt>,
    a: Option<At>,
    alpha: Option<Alphat>,
}

pub struct DHParams<
    Theta: CompliantNumerical + Trig,
    D: CompliantNumerical + Trig,
    A: CompliantNumerical + Trig,
    Alpha: CompliantNumerical + Trig,
> {
    theta: Theta,
    d: D,
    a: A,
    alpha: Alpha,
}
impl<
        const THETA: bool,
        const D: bool,
        const A: bool,
        const ALPHA: bool,
        Thetat: CompliantNumerical + Trig,
        Dt: CompliantNumerical + Trig,
        At: CompliantNumerical + Trig,
        Alphat: CompliantNumerical + Trig,
    > Default for DHBuilder<THETA, D, A, ALPHA, Thetat, Dt, At, Alphat>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        const THETA: bool,
        const D: bool,
        const A: bool,
        const ALPHA: bool,
        Thetat: CompliantNumerical + Trig,
        Dt: CompliantNumerical + Trig,
        At: CompliantNumerical + Trig,
        Alphat: CompliantNumerical + Trig,
    > DHBuilder<THETA, D, A, ALPHA, Thetat, Dt, At, Alphat>
{
    pub fn new() -> Self {
        Self {
            theta: None,
            d: None,
            a: None,
            alpha: None,
        }
    }
}

impl<
        const D: bool,
        const A: bool,
        const ALPHA: bool,
        Thetat: CompliantNumerical + Trig,
        Dt: CompliantNumerical + Trig,
        At: CompliantNumerical + Trig,
        Alphat: CompliantNumerical + Trig,
    > DHBuilder<false, D, A, ALPHA, Thetat, Dt, At, Alphat>
{
    pub fn theta(self, val: Thetat) -> DHBuilder<true, D, A, ALPHA, Thetat, Dt, At, Alphat> {
        DHBuilder {
            theta: Some(val),
            d: self.d,
            a: self.a,
            alpha: self.alpha,
        }
    }
}

impl<
        const THETA: bool,
        const A: bool,
        const ALPHA: bool,
        Thetat: CompliantNumerical + Trig,
        Dt: CompliantNumerical + Trig,
        At: CompliantNumerical + Trig,
        Alphat: CompliantNumerical + Trig,
    > DHBuilder<THETA, false, A, ALPHA, Thetat, Dt, At, Alphat>
{
    pub fn d(self, val: Dt) -> DHBuilder<THETA, true, A, ALPHA, Thetat, Dt, At, Alphat> {
        DHBuilder {
            theta: self.theta,
            d: Some(val),
            a: self.a,
            alpha: self.alpha,
        }
    }
}

impl<
        const THETA: bool,
        const D: bool,
        const ALPHA: bool,
        Thetat: CompliantNumerical + Trig,
        Dt: CompliantNumerical + Trig,
        At: CompliantNumerical + Trig,
        Alphat: CompliantNumerical + Trig,
    > DHBuilder<THETA, D, false, ALPHA, Thetat, Dt, At, Alphat>
{
    pub fn a(self, val: At) -> DHBuilder<THETA, D, true, ALPHA, Thetat, Dt, At, Alphat> {
        DHBuilder {
            theta: self.theta,
            d: self.d,
            a: Some(val),
            alpha: self.alpha,
        }
    }
}
impl<
        const THETA: bool,
        const D: bool,
        const A: bool,
        Thetat: CompliantNumerical + Trig,
        Dt: CompliantNumerical + Trig,
        At: CompliantNumerical + Trig,
        Alphat: CompliantNumerical + Trig,
    > DHBuilder<THETA, D, A, false, Thetat, Dt, At, Alphat>
{
    pub fn alpha(self, val: Alphat) -> DHBuilder<THETA, D, A, true, Thetat, Dt, At, Alphat> {
        DHBuilder {
            theta: self.theta,
            d: self.d,
            a: self.a,
            alpha: Some(val),
        }
    }
}

impl<
        Thetat: CompliantNumerical + Trig,
        Dt: CompliantNumerical + Trig,
        At: CompliantNumerical + Trig,
        Alphat: CompliantNumerical + Trig,
    > DHBuilder<true, true, true, true, Thetat, Dt, At, Alphat>
{
    pub fn complete(self) -> DHParams<Thetat, Dt, At, Alphat> {
        DHParams {
            theta: self.theta.unwrap(),
            d: self.d.unwrap(),
            a: self.a.unwrap(),
            alpha: self.alpha.unwrap(),
        }
    }
    pub fn to_table(&self) -> DHTable
    where
        Thetat: ToTex,
        Dt: ToTex,
        At: ToTex,
        Alphat: ToTex,
    {
        DHTable::new(
            &self.theta.clone().unwrap(),
            &self.d.clone().unwrap(),
            &self.a.clone().unwrap(),
            &self.alpha.clone().unwrap(),
        )
    }
}

impl<
        Theta: CompliantNumerical + Trig,
        D: CompliantNumerical + Trig,
        A: CompliantNumerical + Trig,
        Alpha: CompliantNumerical + Trig,
    > DHParams<Theta, D, A, Alpha>
where
    Theta: Mul<D, Output = Theta> + Mul<A, Output = Theta> + Mul<Alpha, Output = Theta>,
{
    pub fn pose<const PREV: usize, const CURR: usize>(
        self,
    ) -> Result<Pose<Theta, PREV, CURR>, Error> {
        let rz = Pose::<Theta, PREV, PREV>::rot(rotz(self.theta)?);
        let tz: Pose<D, PREV, PREV> = Pose::from_translation(Vector::new_from_data(
            [D::zero(), D::zero(), self.d].clone(),
        ));

        let tx = Pose::<A, PREV, PREV>::from_translation(Vector::new_from_data(
            [self.a, A::zero(), A::zero()].clone(),
        ));
        let rx = Pose::<Alpha, PREV, CURR>::rot(rotx(self.alpha)?);
        Ok(rz * tz * tx * rx)
    }
}
