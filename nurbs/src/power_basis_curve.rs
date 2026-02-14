use crate::curve::Curve2D;
use crate::curve::Curve3D;
use crate::interval::Interval;
use crate::point_types::Point2D;
use crate::point_types::Point3D;

pub struct PowerBasisCurve2D {
    bases: Vec<Point2D>,
    curve_interval: Interval,
}

pub struct PowerBasisCurve3D {
    bases: Vec<Point3D>,
    curve_interval: Interval,
}

impl PowerBasisCurve2D {
    pub fn new(bases: Vec<Point3D>, curve_interval: Interval) -> PowerBasisCurve2D{
        PowerBasisCurve2D {bases, curve_interval}
    }
    
    pub fn from_bases(bases: Vec<Point3D>) -> PowerBasisCurve2D{
        PowerBasisCurve2D {bases, curve_interval: Interval::new(Point2D::new([0.0, 1.0]))}
    }

    fn horner(u: f64) -> Point2D {

    }
}


impl Curve2D for PowerBasisCurve2D {
    fn interval(&self) -> &Interval {
        &self.curve_interval
    }

    fn evaluate(&self, parameter: f64) -> Point2D {
        let param = self.curve_interval.clamp_value(parameter);
        Point2D::new([param, param])
    }
}
