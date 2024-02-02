use matrs::matrix::helpers::rotations::*;
use num_traits::Zero;

use robotics::{decore::decorators::ToTex, link::DHBuilder, pose::Pose, sym, syms::*};

fn task_1() {
    let pose: Pose<Sym, 0, 1> = Pose::rot(rotz(Sym::zero()).unwrap());
    let p1_dh = DHBuilder::new()
        .a(sym!("a_1"))
        .theta(sym!("q_1"))
        .d(0f32)
        .alpha(0f32);
    let table = p1_dh.to_table();
    let p1: Pose<Sym, 1, 2> = p1_dh.complete().pose().unwrap();

    let p2_dh = DHBuilder::new()
        .a(sym!("a_2"))
        .theta(sym!("q_2"))
        .d(0f32)
        .alpha(Sym::zero()-sym!(Constant::Pi));
    let table = table.extend(p2_dh.to_table());
    let p2: Pose<Sym, 2, 3> = p2_dh.complete().pose().unwrap();

    let p3_dh = DHBuilder::new()
        .a(sym!("a_3"))
        .theta(sym!("q_3"))
        .d(Sym::zero())
        .alpha(sym!(Constant::Pi));
    let table = table.extend(p3_dh.to_table());

    let p3: Pose<Sym, 3, 4> = p3_dh.complete().pose().unwrap(); 
    let p02 = (pose*p1.clone() * p2.clone()).opt();
    let p03 = (p02.clone() * p3.clone()).opt();
    println!("{}\n{}\n{}",p1.to_tex(Some("^0T_1")),p02.to_tex(Some("^0T_2")),p03.to_tex(Some("^0T_3")));
    println!("fk:\n{}", p03.fk().to_matrix().to_tex(Some("fk")));
    
    println!("{}",table.to_tex());
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
        .alpha((Sym::zero() - Sym::Constant(Constant::Pi)) / 2f32);
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
        .alpha((Sym::zero() - Sym::Constant(Constant::Pi)) / 2f32);
    let table = table.extend(p5_dh.to_table());
    let p5: Pose<Sym, 5, 6> = p5_dh.complete().pose().unwrap();

    let p6_dh = DHBuilder::new()
        .a(0f32)
        .theta(sym!("q_6"))
        .d(0f32)
        .alpha(0f32);
    let table = table.extend(p6_dh.to_table());
    let p6: Pose<Sym, 6, 7> = p6_dh.complete().pose().unwrap();

    // This is where the idea falls appart. I should probably use matlab for this. Or write some
    // better
    // optimizations
    let p02 = (pose*p1.clone() * p2.clone()).opt();
    let p03 = (p02.clone() * p3.clone()).opt();
    let p04 = (p03.clone() * p4.clone()).opt();
    let p05 = (p04.clone() * p5.clone()).opt();
    let p06 = (p05.clone() * p6.clone()).opt();
    println!("{}", p1.to_tex(Some("^0T_1")));
    println!("{}", p02.to_tex(Some("^0T_2")));
    println!("{}", p03.to_tex(Some("^0T_3")));
    println!("{}", p04.to_tex(Some("^0T_4")));
    println!("{}", p05.to_tex(Some("^0T_5")));
    println!("{}", p06.to_tex(Some("^0T_6")));
    println!("{}", p06.fk().to_matrix().to_tex(Some("fk")));
    println!("DHParams: \n{}", table.to_tex());
}



fn task_3(){
    let p0_dh = DHBuilder::new().theta(sym!("\\theta_1")).a(sym!("a_1")).d(0f32).alpha(0f32);
    let table = p0_dh.to_table();
    let p0:Pose<Sym,0,1> = p0_dh.complete().pose().unwrap();

    let p1_dh = DHBuilder::new().theta(sym!("\\theta_2")).a(sym!("a_2")).d(0f32).alpha(0f32);
    let table = table.extend(p1_dh.to_table());
    let p1:Pose<Sym,1,2> = p1_dh.complete().pose().unwrap();

    let p2_dh = DHBuilder::new().theta(sym!(0f32)).a(0f32).d(sym!("d_3")).alpha(sym!(Constant::Pi));
    let table = table.extend(p2_dh.to_table());
    let p2:Pose<Sym,2,3> = p2_dh.complete().pose().unwrap();

    let p3_dh = DHBuilder::new().theta(sym!("\\theta_4")).a(0f32).d(sym!("d_4")).alpha(0f32);
    let table = table.extend(p3_dh.to_table());
    let p3:Pose<Sym,3,4> = p3_dh.complete().pose().unwrap();


    let p01 = (p0.clone()*p1).opt();
    let p02 = (p01.clone()*p2).opt();
    let p03 = (p02.clone()*p3).opt();
    println!("DHParams: \n{}", table.to_tex());
    println!("P0 : \n{}",p0.to_tex(Some("^0T_1")));
    println!("P1 : \n{}",p01.to_tex(Some("^0T_2")));
    println!("P2 : \n{}",p02.to_tex(Some("^0T_3")));
    println!("P3 : \n{}",p03.to_tex(Some("^0T_4")));
    println!("fk : \n{}",p03.fk().to_matrix().to_tex(Some("fk")));






}


fn main() {
    task_1();
    task_2();
    task_3()
}
