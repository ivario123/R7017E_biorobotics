use core::f32::consts::PI;
use matrs::matrix::helpers::rotations::*;
use matrs::predule::*;

use robotics::{
    decore::decorators::{ToMatlab, ToTex},
    pose::Pose,
};

fn task4() {
    let p0_1 = Pose::<f32,0, 1>::from_translation(Vector::new_from_data([3f32, 0f32, 0f32]));
    let p1_2 = Pose::<f32,1, 2>::rot(rotz(PI / 2f32).unwrap());
    let (rot, _t) = p1_2.clone().decompose();

    let translation = rot.clone().transpose() * Vector::new_from_data([0f32, 1f32, 0f32]);
    let p2_3 = Pose::<f32,2, 3>::from_translation(translation);
    println!("p2_3:\n{p2_3}");

    // Composite transform calculation
    let p0_2 = p0_1.clone() * p1_2.clone();
    let p0_3 = p0_1.clone() * p1_2.clone() * p2_3.clone();

    // let mut plot = Plot::new();
    // let (p0_1_t,_) = p0_1.clone().plot();
    // let (p1_2_t,_) = (p0_1.clone()*p1_2.clone()).plot();
    // let (p2_3_t,_) = (p0_1*p1_2*p2_3).plot();
    //
    // plot.add_traces(p0_1_t);
    // plot.add_traces(p1_2_t);
    // plot.add_traces(p2_3_t);
    // plot.show();

    println!("p0_1:\n{}", p0_1);
    println!("p0_2:\n{}", p0_2);
    println!("p0_3:\n{}", p0_3);
    println!("Matlab: \n{}", p0_3.to_matlab("pose_0_3"));
    println!(
        "TeX: \n{}\n{}\n{}",
        p0_1.to_tex(Some("^0T_1")),
        p0_2.to_tex(Some("^0T_2")),
        p0_3.to_tex(Some("^0T_3"))
    )
}

fn task3() {
    let r1 = rotz(PI / 3f32).unwrap();
    let r2 = rotx(PI / 6f32).unwrap();
    println!("TeX");
    println!(
        "{}\n{}\n{}",
        r1.to_tex(Some("R_1")),
        r2.to_tex(Some("R_2")),
        (r1.clone() * r2.clone()).to_tex(Some("R_{composite}"))
    );
    println!(".m");
    println!("{}", (r1 * r2).to_matlab("R"))
    // This matrix math library can't calculate eigen vectors by design, it is meerely ment as a
    // forwards propagation layer for low power systems. so task 3 has to be left to the reader.
}

fn main() {
    task3();
    task4()
}
