use core::f32::consts::PI;
use matrs::matrix::helpers::rotations::*;
use matrs::predule::*;
use num_traits::Zero;
use plotly::{Layout, Plot};
use robotics::{
    decore::decorators::{ToMatlab, ToTex},
    graphing::Plottable,
    link::DHBuilder,
    pose::Pose,
    sym,
    syms::*,
};

fn task_1() {
    let a1 = sym!("a1");
    let a2 = sym!("a2");
    let a3 = sym!("a3");
    let q1 = sym!("q1");
    let q2 = sym!("q2");
    let q3 = sym!("q3");
    let p01 = Pose::<Sym, 0, 1>::from_dh(Sym::zero(), q1, a1, 0f32).unwrap();
    let p12 = Pose::<Sym, 1, 2>::from_dh(Sym::zero(), q2, a2, 0f32).unwrap();
    let p23 = Pose::<Sym, 2, 3>::from_dh(Sym::zero(), q3, a3, 0f32).unwrap();

    let p02 = p01.clone() * p12.clone();
    let p03 = p02.clone() * p23.clone();
    println!("{p01}\n{p02}\n{p03}");
    println!("fk:\n{}", p03.fk().to_matrix());
}

fn task_2() {
    let pose:Pose<Sym,0,1> = Pose::rot(rotz(Sym::zero()).unwrap());
    let p1: Pose<Sym, 1, 2> = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q1"))
        .d(0f32)
        .alpha(PI / 2f32)
        .complete()
        .pose()
        .unwrap();
    let p2: Pose<Sym, 2, 3> = DHBuilder::new()
        .a(sym!("a2"))
        .theta(sym!("q2"))
        .d(0f32)
        .alpha(0f32)
        .complete()
        .pose()
        .unwrap();
    let p3: Pose<Sym, 3, 4> = DHBuilder::new()
        .a(sym!("a3"))
        .theta(sym!("q3"))
        .d(sym!("d3"))
        .alpha(-PI / 2f32)
        .complete()
        .pose()
        .unwrap();
    let p4: Pose<Sym, 4, 5> = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q4"))
        .d(sym!("d4"))
        .alpha(PI / 2f32)
        .complete()
        .pose()
        .unwrap();
    let p5: Pose<Sym, 5, 6> = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q4"))
        .d(0f32)
        .alpha(-PI / 2f32)
        .complete()
        .pose()
        .unwrap();
    let p6: Pose<Sym, 6, 7> = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q4"))
        .d(0f32)
        .alpha(0f32)
        .complete()
        .pose()
        .unwrap();


    println!("{p1}\n{p2}\n{p3}\n{p4}\n{p5}\n{p6}");
    // This is where the idea falls appart. I should probably use matlab for this. Or write some
    // better
    // optimizations
    let res = (pose*p1*p2*p3*p4*p5*p6).opt().opt();
    println!("{}",res);
    println!("{}",res.fk().to_matrix());

}

fn main() {
    task_1();
    task_2()

}
