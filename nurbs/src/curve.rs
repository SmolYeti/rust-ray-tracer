use crate::interval::Interval;
use crate::point_types::Point2D;
use crate::point_types::Point3D;

pub trait Curve2D {
    fn evaluate_curve(&self, parameter: f64) -> Point2D;

    pub fn evaluate_curve_points(&self, point_count: u32, interval: Interval) -> vec<Point2D> {
        let mut points = vec::with_capacity(point_count); 

        let div = (interval.range.y() - interval.range.x()) / ((point_count - 1) as f64);
        for n .. point_count {
            let parameter = interval.range.x() + (n as f64 * div);
            points.push(self.evaluate_curve(parameter));
        }
        points
    }
}

pub trait Curve3D {
    fn evaluate_curve(&self, parameter: f64) -> Point3D;

    pub fn evaluate_curve_points(&self, point_count: u32, interval: Interval) -> vec<Point3D> {
        let mut points = vec::with_capacity(point_count); 

        let div = (interval.range.y() - interval.range.x()) / ((point_count - 1) as f64);
        for n .. point_count {
            let parameter = interval.range.x() + (n as f64 * div);
            points.push(self.evaluate_curve(parameter));
        }
        points
    }
}