use crate::interval::Interval;
use crate::point_types::Point2D;
use crate::point_types::Point3D;

pub trait Curve2D {
    fn interval(&self) -> Interval;

    fn evaluate(&self, parameter: f64) -> Point2D;

    fn evaluate_points(&self, point_count: usize) -> Vec<Point2D> {
        let mut points = Vec::<Point2D>::with_capacity(point_count); 

        let div = (self.interval().range.y() - self.interval().range.x()) / ((point_count - 1) as f64);
        for n in 0..point_count {
            let parameter = self.interval().range.x() + (n as f64 * div);
            points.push(self.evaluate(parameter));
        }
        points
    }
}

pub trait Curve3D {
    fn interval(&self) -> Interval;

    fn evaluate(&self, parameter: f64) -> Point3D;

    fn evaluate_points(&self, point_count: usize) -> Vec<Point3D> {
        let mut points = Vec::<Point3D>::with_capacity(point_count); 

        let div = (self.interval().range.y() - self.interval().range.x()) / ((point_count - 1) as f64);
        for n in 0..point_count {
            let parameter = self.interval().range.x() + (n as f64 * div);
            points.push(self.evaluate(parameter));
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use crate::curve::Curve2D;
    use crate::curve::Curve3D;
    use crate::interval::Interval;
    use crate::point_types::Point2D;
    use crate::point_types::Point3D;
    use crate::utility::f64_equal;

    pub struct ImplCurve2D {
        curve_interval: Interval,        
    }

    impl Curve2D for ImplCurve2D {
        fn interval(&self) -> Interval {
            self.curve_interval
        }

        fn evaluate(&self, parameter: f64) -> Point2D {
            let param = self.curve_interval.clamp_value(parameter);
            Point2D::new([param, param])
        }
    }

    impl ImplCurve2D {
        pub fn new(curve_interval: Interval) -> ImplCurve2D{
            ImplCurve2D {curve_interval}
        }
    }

    pub struct ImplCurve3D {
        curve_interval: Interval,        
    }

    impl Curve3D for ImplCurve3D {
        fn interval(&self) -> Interval {
            self.curve_interval
        }

        fn evaluate(&self, parameter: f64) -> Point3D {
            let param = self.curve_interval.clamp_value(parameter);
            Point3D::new([param, param, param])
        }
    }

    impl ImplCurve3D {
        pub fn new(curve_interval: Interval) -> ImplCurve3D{
            ImplCurve3D {curve_interval}
        }
    }

    #[test]
    fn test_curve2d() {
        let curve = ImplCurve2D::new(Interval::new(Point2D::new([1.0, 2.0])));

        let interval = curve.interval();
        assert!(f64_equal(interval.range.x(), 1.0));
        assert!(f64_equal(interval.range.y(), 2.0));

        let eval = curve.evaluate(1.2);
        assert!(f64_equal(eval.x(), 1.2));
        assert!(f64_equal(eval.y(), 1.2));

        let points = curve.evaluate_points(10);

        assert_eq!(points.len(), 10);
        for n in 0..10 {
            let test_val = 1.0 + (n as f64 * (1.0 / 9.0));
            assert!(f64_equal(points[n].x(), test_val), "Value was {} vs {}", points[n].x(), test_val);
            assert!(f64_equal(points[n].y(), test_val), "Value was {} vs {}", points[n].y(), test_val);
        }
    }

    #[test]
    fn test_curve3d() {
        let curve = ImplCurve3D::new(Interval::new(Point2D::new([1.0, 2.0])));

        let interval = curve.interval();
        assert!(f64_equal(interval.range.x(), 1.0));
        assert!(f64_equal(interval.range.y(), 2.0));

        let eval = curve.evaluate(1.2);
        assert!(f64_equal(eval.x(), 1.2));
        assert!(f64_equal(eval.y(), 1.2));
        assert!(f64_equal(eval.z(), 1.2));

        let points = curve.evaluate_points(10);

        assert_eq!(points.len(), 10);
        for n in 0..10 {
            let test_val = 1.0 + (n as f64 * (1.0 / 9.0));
            assert!(f64_equal(points[n].x(), test_val), "Value was {} vs {}", points[n].x(), test_val);
            assert!(f64_equal(points[n].y(), test_val), "Value was {} vs {}", points[n].y(), test_val);
            assert!(f64_equal(points[n].z(), test_val), "Value was {} vs {}", points[n].z(), test_val);
        }
    }
}