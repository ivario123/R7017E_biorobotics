use super::Matrix;

pub mod decorators {
    use std::fmt::Display;

    use matrs::{matrix::rotations::Trig, vec::Vector, CompliantNumerical};

    use crate::pose::{Coord, Pose};

    use super::*;
    pub trait ToMatlab {
        fn to_matlab(&self, identifier: &'static str) -> String;
    }
    pub trait ToTex {
        fn to_tex(&self, identifier: Option<&'static str>) -> String;
    }
    impl<T: CompliantNumerical + Display, const M: usize, const N: usize> ToMatlab for Matrix<T, M, N> {
        fn to_matlab(&self, identifier: &'static str) -> String {
            let mut ret = format!("{identifier}\t=\t[");
            for row in self.get_elements().into_iter() {
                ret += "[";
                let max = row.len() - 1;
                for (idx, el) in row.iter().enumerate() {
                    if idx < max {
                        ret += format!("{el:.4},").as_str();
                    } else {
                        ret += format!("{el:.4}").as_str();
                    }
                }
                ret += "];";
            }
            ret += "];\n";
            ret
        }
    }
    impl<T: CompliantNumerical + ToTex, const M: usize, const N: usize> ToTex for Matrix<T, M, N> {
        fn to_tex(&self, identifier: Option<&'static str>) -> String {
            let has_identifer = identifier.is_some();
            let identifier = identifier.unwrap_or("");
            let mut ret = format!("\\begin{{equation}}\\label{{ {identifier} }}\n\t");
            if has_identifer {
                ret += format!("{identifier} = ").as_str();
            }
            ret += format!("\\left[\\begin{{array}}{{{}}}\n", "c".repeat(N)).as_str();
            for row in self.get_elements().into_iter() {
                let max = row.len() - 1;
                ret += "\t\t";

                for (idx, el) in row.iter().enumerate() {
                    if idx < max {
                        ret += format!("{} & ",el.to_tex(None)).as_str();
                    } else {
                        ret += format!("{}",el.to_tex(None)).as_str();
                    }
                }
                ret += "\\\\\n";
            }
            ret += "\t\\end{array}\\right]\n";
            ret += "\\end{equation}\n";
            ret
        }
    }

    impl<'a, T: CompliantNumerical + Display, const M: usize, const N: usize> ToMatlab
        for &'a Matrix<T, M, N>
    {
        fn to_matlab(&self, identifier: &'static str) -> String {
            let mut ret = format!("{identifier}\t=\t[");
            for row in self.get_elements().into_iter() {
                ret += "[";
                let max = row.len() - 1;
                for (idx, el) in row.iter().enumerate() {
                    if idx < max {
                        ret += format!("{el},").as_str();
                    } else {
                        ret += format!("{el}").as_str();
                    }
                }
                ret += "];";
            }
            ret += "];\n";
            ret
        }
    }

    impl<T: CompliantNumerical + ToTex, const COUNT: usize> ToTex for Vector<T, COUNT> {
        fn to_tex(&self, identifier: Option<&'static str>) -> String {
            self.clone().to_matrix().to_tex(identifier)
        }
    }

    impl<T: CompliantNumerical + ToTex, const FRAME: usize> ToTex for Coord<T, FRAME> {
        fn to_tex(&self, identifier: Option<&'static str>) -> String {
            self.rpr.to_tex(identifier)
        }
    }

    impl<T: CompliantNumerical + Display + Trig, const PREV: usize, const CURR: usize> ToMatlab
        for Pose<T, PREV, CURR>
    {
        fn to_matlab(&self, identifier: &'static str) -> String {
            let intermediate: &Matrix<T, 4, 4> = self.into();

            intermediate.to_matlab(identifier)
        }
    }
    impl<T: CompliantNumerical + ToTex + Trig, const PREV: usize, const CURR: usize> ToTex
        for Pose<T, PREV, CURR>
    {
        fn to_tex(&self, identifier: Option<&'static str>) -> String {
            let intermediate: &Matrix<T, 4, 4> = self.into();

            intermediate.to_tex(identifier)
        }
    }
    impl ToTex for f32{
        fn to_tex(&self, _identifier: Option<&'static str>) -> String {
            self.to_string()
        }
    }
}
