use core::f32::consts::PI;
use matrs::matrix::helpers::rotations::*;
use matrs::predule::*;
use num_traits::Zero;

use robotics::{
    decore::decorators::{ToMatlab, ToTex},
    pose::Pose,
    syms::*,
};

fn main() {
    let p1: Pose<Sym, 0, 1> = Pose::rot(rotz(Sym::Identifier("theta")).unwrap());
    let p2: Pose<f32, 1, 2> = Pose::rot(rotx(PI / 2f32).unwrap());
    let p3: Pose<Sym, 2, 3> = Pose::from_translation(Vector::new_from_data([
        Sym::zero(),
        Sym::zero(),
        Sym::Identifier("l_1"),
    ]));
    let p0_3 = p1.clone() * p2.clone() * p3;
    println!("{}", p1);
    println!("{}", p1.to_tex(Some("^0P_1")));

    println!("{}", p0_3);
    println!("{}", p0_3.to_tex(Some("^0P_3")));
    println!("{}", p0_3.to_matlab("P_3"));
}
