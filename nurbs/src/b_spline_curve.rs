use crate::curve::Curve2D;
use crate::curve::Curve3D;
use crate::interval::Interval;
use crate::point_types::Point;
use crate::point_types::Point2D;
use crate::point_types::Point3D;

pub struct BSplineCurveND<const N: usize> {
    degree: u32,
    control_points: Vec<Point<N>>,
    knots: Vec<f64>,
    curve_interval: Interval,
}

pub type BSplineCurve2D = BSplineCurveND<2>;
pub type BSplineCurve3D = BSplineCurveND<3>;

impl<const N: usize> BSplineCurveND<N> {
    pub fn new(
        degree: u32,
        control_points: Vec<Point<N>>,
        knots: Vec<f64>,
        curve_interval: Interval,
    ) -> PowerBasisCurveND<N> {
        PowerBasisCurveND {
            degree,
            control_points,
            knots,
            curve_interval,
        }
    }

    // TODO: Implement knot methods before implementing these methods
}
