use core::f32::consts::PI;
use matrs::matrix::helpers::rotations::*;
use matrs::predule::*;
use plotly::{Layout, Plot};
use robotics::{graphing::Plottable, pose::Pose};

fn main() {
    let (wf, _, _) = Pose::<0, 0>::from_angles(Vector::new()).unwrap();
    let (pose, axis, r) = Pose::<0, 1>::from_angles(Vector::new_from_data([
        PI / 3f32,
        PI / 6f32,
        2f32 * PI / 3f32,
    ]))
    .unwrap();
    println!("Rotation axis : \n{:}", axis.to_matrix());
    println!("Rotation matrix : \n{:}", r);

    println!("{}", pose);
    println!("{}", rotx(0f32).unwrap());
    println!("{}", roty(0f32).unwrap());
    println!("{}", rotz(0f32).unwrap());
    let pose: Pose<0, 1> = Pose::rot(rotz(PI / 3f32).unwrap() * rotx(PI / 6f32).unwrap());

    let mut plot = Plot::new();

    let (wf_p, wf_a) = wf.plot();
    let (p_p, p_a) = pose.plot();

    plot.add_traces(wf_p);
    plot.add_traces(p_p);
    let layout = Layout::new().annotations(wf_a);
    plot.set_layout(layout);
    plot.show()
}
