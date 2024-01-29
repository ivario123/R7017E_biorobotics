use crate::pose::{Coord, Pose};
use matrs::vec::Vector;
use plotly::common::{LegendGroupTitle, TextPosition};
use plotly::plot::{Trace, Traces};
use plotly::{common::TextAnchor, layout::Annotation, Scatter3D};

pub trait Plottable {
    fn plot(&self) -> (Vec<Box<dyn Trace>>, Vec<Annotation>);
}
// Any coordinate expressed in the worldframe is plottable as a point
impl Plottable for Coord<f32, 0> {
    fn plot(&self) -> (Vec<Box<dyn Trace>>, Vec<Annotation>) {
        let mut ret = Vec::new();

        let el: Box<dyn Trace> =
            Scatter3D::new(vec![self.rpr[0]], vec![self.rpr[1]], vec![self.rpr[2]]);
        ret.push(el);
        (ret, Vec::new())
    }
}

fn plot_vec(
    ret: &mut Vec<Box<dyn Trace>>,
    annotations: &mut Vec<Annotation>,
    start: Vector<f32, 3>,
    end: Vector<f32, 3>,
    identifier: &str,
) {
    let xs = vec![start[0], end[0]];
    let ys = vec![start[1], end[1]];
    let zs = vec![start[2], end[2]];
    let mut res = Scatter3D::new(xs, ys, zs)
        .mode(plotly::common::Mode::Lines)
        .legend_group_title(LegendGroupTitle::new(identifier));
    ret.append(&mut vec![res.text("end")])
}

impl<const FRAME: usize> Plottable for Pose<f32, 0, FRAME> {
    fn plot(&self) -> (Vec<Box<dyn Trace>>, Vec<Annotation>) {
        let (origin, x, y, z) = self.base_vectors();
        let mut ret = Vec::new();
        let mut annotations = Vec::new();

        plot_vec(&mut ret, &mut annotations, origin, x, "x");
        plot_vec(&mut ret, &mut annotations, origin, y, "y");
        plot_vec(&mut ret, &mut annotations, origin, z, "z");
        (ret, annotations)
    }
}
