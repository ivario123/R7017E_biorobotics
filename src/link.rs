use matrs::{
    matrix::helpers::rotations::*, matrix::rotations::Trig, predule::*, CompliantNumerical,
};
use std::fmt::Alignment;
use std::ops::Mul;

use crate::pose::{Coord, Pose};

// pub struct Link<const IDX: usize,L:CompliantNumerical+Trig,> {
//      length:
// }
//
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
