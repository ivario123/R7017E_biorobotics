use std::fmt::Display;

use matrs::matrix::helpers::rotations::{rotx, roty, rotz, Error, Trig};
use matrs::{CompliantNumerical, MatrixInterface, VectorTrait};

use super::{Matrix, Vector};

#[derive(Clone)]
pub struct Coord<T: CompliantNumerical, const FRAME: usize> {
    pub(crate) rpr: Vector<T, 4>,
}

#[derive(Clone, Debug)]
pub struct Pose<T: CompliantNumerical + Trig, const PREV: usize, const CURR: usize> {
    rpr: Matrix<T, 4, 4>,
}

impl<T: CompliantNumerical + Trig, const PREV: usize, const CURR: usize> Pose<T, PREV, CURR> {
    /// Converts to a series of 0..end rotations. For 3D this would be XYZ.
    ///
    /// return the pose, the axis of rotation and the full rotation matrix
    pub fn from_angles(
        angles: Vector<T, 3>,
    ) -> Result<(Self, Vector<T, 3>, Matrix<T, 3, 3>), Error> {
        let mag = angles.magnitude();
        let rot_axis = angles.clone() / mag;
        let rx: Matrix<T, 3, 3> = rotx(angles[0].clone())?;
        let ry: Matrix<T, 3, 3> = roty(angles[1].clone())?;
        let rz: Matrix<T, 3, 3> = rotz(angles[2].clone())?;
        let rxy = rx * ry;
        let rxyz = rxy * rz;
        Ok((Self::rot(rxyz.clone()), rot_axis, rxyz))
    }
    /// Converts a translation vector to a pose
    pub fn from_translation(t: Vector<T, 3>) -> Self {
        Self {
            rpr: Matrix::new_from_data(
                [
                    [T::one(), T::zero(), T::zero(), t[0].clone()].clone(),
                    [T::zero(), T::one(), T::zero(), t[1].clone()].clone(),
                    [T::zero(), T::zero(), T::one(), t[2].clone()].clone(),
                    [T::zero(), T::zero(), T::zero(), T::one()].clone(),
                ]
                .clone(),
            ),
        }
    }
    /// Converts [DH parameters](https://en.wikipedia.org/wiki/Denavit%E2%80%93Hartenberg_parameters)
    /// to a pose
    ///
    /// Returns the composite transformation
    pub fn from_dh<
        D: CompliantNumerical + Trig,
        THETA: CompliantNumerical + Trig,
        R: CompliantNumerical + Trig,
        A: CompliantNumerical + Trig,
    >(
        d: D,
        theta: THETA,
        r: R,
        alpha: A,
    ) -> Result<Pose<THETA, PREV, CURR>, Error>
    where
        THETA: Mul<T, Output = THETA>
            + Mul<D, Output = THETA>
            + Mul<R, Output = THETA>
            + Mul<A, Output = THETA>,
    {
        let rz = Pose::<THETA, PREV, PREV>::rot(rotz(theta)?);
        let tz: Pose<D, PREV, PREV> =
            Pose::from_translation(Vector::new_from_data([D::zero(), D::zero(), d].clone()));
        let tx = Pose::<R, PREV, PREV>::from_translation(Vector::new_from_data(
            [r, R::zero(), R::zero()].clone(),
        ));
        let rx = Pose::<A, PREV, CURR>::rot(rotx(alpha)?);

        Ok(rz * tz * tx * rx)
    }
    pub fn decompose(self) -> (Matrix<T, 3, 3>, Vector<T, 3>) {
        (self.rotation(), self.translation())
    }
    fn translation(&self) -> Vector<T, 3> {
        Vector::new_from_data(
            [
                self.rpr[(3, 0)].clone(),
                self.rpr[(3, 1)].clone(),
                self.rpr[(3, 2)].clone(),
            ]
            .clone(),
        )
    }
    fn rotation(&self) -> Matrix<T, 3, 3> {
        let mut ret = Matrix::new();
        for i in 0..3 {
            for j in 0..3 {
                ret[(i, j)] = self.rpr[(i, j)].clone();
            }
        }
        ret
    }
}

impl<T: CompliantNumerical + Trig, const PREV: usize, const CURR: usize> Pose<T, PREV, CURR> {
    pub fn rot(r: Matrix<T, 3, 3>) -> Self {
        let data = r.get_elements();

        let mut target = <Matrix<T, 4, 4>>::new();
        for (row, el) in data.into_iter().enumerate() {
            for (col, el) in el.iter().enumerate() {
                target[(row, col)] = el.clone();
            }
        }
        target[(3, 3)] = T::one();
        Self { rpr: target }
    }
    pub fn base_vectors(&self) -> (Vector<T, 3>, Vector<T, 3>, Vector<T, 3>, Vector<T, 3>) {
        let (_rot, origin) = (*self).clone().decompose();
        let origin: Coord<T, CURR> = origin.into();
        let origin: Vector<T, 3> = (self * origin).into();

        let x: Coord<T, CURR> =
            Vector::new_from_data([T::one(), T::zero(), T::zero()].clone()).into();
        let y: Coord<T, CURR> =
            Vector::new_from_data([T::zero(), T::one(), T::zero()].clone()).into();
        let z: Coord<T, CURR> =
            Vector::new_from_data([T::zero(), T::zero(), T::one()].clone()).into();

        let x: Vector<T, 3> = (self * x).into();
        let y: Vector<T, 3> = (self * y).into();
        let z: Vector<T, 3> = (self * z).into();
        return (origin, x, y, z);
    }
}

impl<T: CompliantNumerical + Trig, const CURR: usize> Pose<T, 0, CURR> {
    /// Returns the forward kinematics for the system.
    pub fn fk(self) -> Vector<T, 3> {
        let els: [T; 3] = [
            self.rpr.get(0, 3).clone(),
            self.rpr[(1, 3)].clone(),
            self.rpr[(2, 3)].clone(),
        ];
        Vector::new_from_data(els)
    }
}

impl<T: CompliantNumerical + Trig, const FRAME: usize> From<Vector<T, 3>> for Coord<T, FRAME> {
    fn from(value: Vector<T, 3>) -> Self {
        let data = [
            value[0].clone(),
            value[1].clone(),
            value[2].clone(),
            T::zero(),
        ];
        Self {
            rpr: Vector::new_from_data(data),
        }
    }
}
impl<T: CompliantNumerical + Trig, const FRAME: usize> Into<Vector<T, 3>> for Coord<T, FRAME> {
    fn into(self) -> Vector<T, 3> {
        let data = [
            self.rpr[0].clone(),
            self.rpr[1].clone(),
            self.rpr[2].clone(),
        ];
        Vector::new_from_data(data)
    }
}
impl<T: CompliantNumerical + Trig + std::fmt::Display, const PREV: usize, const CURR: usize> Display
    for Pose<T, PREV, CURR>
where
    Matrix<T, 4, 4>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rpr)
    }
}

// Let the type system handle order of operations and coordinate frame transfers
use core::ops::Mul;
impl<
        T: CompliantNumerical + Trig + Mul<TOther, Output = T>,
        TOther: CompliantNumerical + Trig,
        const PREV: usize,
        const CURR: usize,
        const OTHER: usize,
    > Mul<Pose<TOther, CURR, OTHER>> for Pose<T, PREV, CURR>
{
    type Output = Pose<T, PREV, OTHER>;
    fn mul(self, rhs: Pose<TOther, CURR, OTHER>) -> Self::Output {
        Pose {
            rpr: self.rpr * rhs.rpr,
        }
    }
}
impl<
        T: CompliantNumerical + Trig + Mul<TOther, Output = T>,
        TOther: CompliantNumerical + Trig,
        const PREV: usize,
        const CURR: usize,
    > core::ops::Mul<Coord<TOther, CURR>> for &Pose<T, PREV, CURR>
{
    type Output = Coord<T, PREV>;
    fn mul(self, rhs: Coord<TOther, CURR>) -> Self::Output {
        Coord {
            rpr: self.rpr.clone() * rhs.rpr,
        }
    }
}
impl<T: CompliantNumerical + Trig, const REF: usize> core::ops::Add for Coord<T, REF> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            rpr: self.rpr + rhs.rpr,
        }
    }
}
impl<T: CompliantNumerical + Trig, const REF: usize> core::ops::Sub for Coord<T, REF> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            rpr: self.rpr - rhs.rpr,
        }
    }
}

impl<'a, T: CompliantNumerical + Trig, const PREV: usize, const CURR: usize>
    Into<&'a Matrix<T, 4, 4>> for &'a Pose<T, PREV, CURR>
{
    fn into(self) -> &'a Matrix<T, 4, 4> {
        &self.rpr
    }
}
impl<'a, T: CompliantNumerical + Trig, const PREV: usize, const CURR: usize>
    Into<&'a mut Matrix<T, 4, 4>> for &'a mut Pose<T, PREV, CURR>
{
    fn into(self) -> &'a mut Matrix<T, 4, 4> {
        &mut self.rpr
    }
}
