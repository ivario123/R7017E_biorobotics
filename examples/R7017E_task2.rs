use matrs::matrix::helpers::rotations::*;
use matrs::predule::*;
use num_traits::Zero;
use plotly::{Layout, Plot};
use robotics::{
    decore::decorators::{ToMatlab, ToTex},
    graphing::Plottable,
    link::{DHBuilder, DHTable},
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
    let pose: Pose<Sym, 0, 1> = Pose::rot(rotz(Sym::zero()).unwrap());
    let p1_dh = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q_1"))
        .d(0f32)
        .alpha(Sym::Constant(Constant::Pi) / 2f32);
    let table = p1_dh.to_table();
    let p1: Pose<Sym, 1, 2> = p1_dh.complete().pose().unwrap();

    let p2_dh = DHBuilder::new()
        .a(sym!("a_2"))
        .theta(sym!("q_2"))
        .d(0f32)
        .alpha(0f32); 
    let table = table.extend(p2_dh.to_table());
    let p2: Pose<Sym, 2, 3> = p2_dh.complete().pose().unwrap();

    let p3_dh = DHBuilder::new()
        .a(sym!("a_3"))
        .theta(sym!("q_3"))
        .d(sym!("d_3"))
        .alpha((Sym::Number(0f32)-Sym::Constant(Constant::Pi)) / 2f32);
    let table = table.extend(p3_dh.to_table());
    let p3: Pose<Sym, 3, 4> = p3_dh.complete().pose().unwrap();

    let p4_dh = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q_4"))
        .d(sym!("d_4"))
        .alpha(Sym::Constant(Constant::Pi) / 2f32);
    let table = table.extend(p4_dh.to_table());
    let p4: Pose<Sym, 4, 5> = p4_dh.complete().pose().unwrap();

    let p5_dh = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q_5"))
        .d(0f32)
        .alpha((Sym::Number(0f32)-Sym::Constant(Constant::Pi)) / 2f32);
    let table = table.extend(p5_dh.to_table());
    let p5: Pose<Sym, 5, 6> = p5_dh.complete().pose().unwrap();

    let p6_dh = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q_6"))
        .d(0f32)
        .alpha(0f32);
    let table = table.extend(p6_dh.to_table());
    let p6: Pose<Sym, 6, 7> = p6_dh.complete().pose().unwrap();

    println!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            p1.to_tex(Some("^0T_1")),
            p2.to_tex(Some("^1T_2")),
            p3.to_tex(Some("^2T_3")),
            p4.to_tex(Some("^3T_4")),
            p5.to_tex(Some("^4T_5")),
            p6.to_tex(Some("^5T_6")),
             );
    // This is where the idea falls appart. I should probably use matlab for this. Or write some
    // better
    // optimizations
    let res = ((((((pose * p1).opt() * p2).opt() * p3).opt() * p4).opt() * p5).opt() * p6)
        .opt()
        .opt();
    println!("{}", res);
    println!("{}", res.fk().to_matrix());
    println!("DHParams: \n{}",table.to_tex());
}

fn main() {
    task_1();
    task_2()
}
