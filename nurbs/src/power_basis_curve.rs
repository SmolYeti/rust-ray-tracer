use crate::curve::Curve2D;
use crate::curve::Curve3D;
use crate::interval::Interval;
use crate::point_types::Point;
use crate::point_types::Point2D;
use crate::point_types::Point3D;

pub struct PowerBasisCurveND<const N: usize> {
    bases: Vec<Point<N>>,
    curve_interval: Interval,
}

pub type PowerBasisCurve2D = PowerBasisCurveND<2>;
pub type PowerBasisCurve3D = PowerBasisCurveND<3>;

impl<const N: usize> PowerBasisCurveND<N> {
    pub fn new(bases: Vec<Point<N>>, curve_interval: Interval) -> PowerBasisCurveND<N> {
        PowerBasisCurveND {
            bases,
            curve_interval,
        }
    }

    pub fn from_bases(bases: Vec<Point<N>>) -> PowerBasisCurveND<N> {
        PowerBasisCurveND::new(bases, Interval::new(Point2D::new([0.0, 1.0])))
    }

    fn horner(&self, u: f64) -> Point<N> {
        // Chaper 1, Algorithm 1.1 Horner 1, p7
        let mut point = Point::empty();
        if !self.bases.is_empty() {
            point = self.bases[self.bases.len() - 1];
        }

        let mut i = self.bases.len() as i32 - 2;
        while i >= 0 {
            point = (u * point) + self.bases[i as usize];
            i -= 1;
        }

        point
    }
}

impl Curve2D for PowerBasisCurve2D {
    fn interval(&self) -> &Interval {
        &self.curve_interval
    }

    fn evaluate(&self, parameter: f64) -> Point2D {
        self.horner(parameter)
    }
}

impl Curve3D for PowerBasisCurve3D {
    fn interval(&self) -> &Interval {
        &self.curve_interval
    }

    fn evaluate(&self, parameter: f64) -> Point3D {
        self.horner(parameter)
    }
}

#[cfg(test)]
mod tests {
    use crate::curve::Curve2D;
    use crate::curve::Curve3D;
    use crate::point_types::Point2D;
    use crate::point_types::Point3D;
    use crate::power_basis_curve::PowerBasisCurve2D;
    use crate::power_basis_curve::PowerBasisCurve3D;
    use crate::utility::f64_equal;
    use crate::utility::f64_near;

    #[test]
    fn test_2d_construct() {
        let bases = Vec::<Point2D>::new();
        let curve = PowerBasisCurve2D::from_bases(bases);
        let point = curve.evaluate(0.5);
        assert!(f64_equal(point.x(), 0.0));
        assert!(f64_equal(point.y(), 0.0));
    }

    #[test]
    fn test_2d_point() {
        let mut bases = Vec::<Point2D>::with_capacity(3);
        bases.push(Point2D::new([0.0, 0.0]));
        bases.push(Point2D::new([1.0, 2.0]));
        bases.push(Point2D::new([2.0, 0.0]));
        let curve = PowerBasisCurve2D::from_bases(bases);
        let point = curve.evaluate(0.5);
        assert!(f64_equal(point.x(), 1.0));
        assert!(f64_equal(point.y(), 1.0));
    }

    #[test]
    fn test_2d_points() {
        let mut bases = Vec::<Point2D>::with_capacity(3);
        bases.push(Point2D::new([0.0, 0.0]));
        bases.push(Point2D::new([1.0, 2.0]));
        bases.push(Point2D::new([2.0, 0.0]));
        let curve = PowerBasisCurve2D::from_bases(bases);

        let points = curve.evaluate_points(100);

        let p0 = Point2D::new([0.0, 0.0]);
        let p1 = Point2D::new([1.0, 2.0]);
        let p2 = Point2D::new([2.0, 0.0]);
        let div = 1.0 / 99.0;
        for i in 0..100 {
            let param = i as f64 * div;
            let test_point = p0 + (p1 * param) + (p2 * param * param);

            assert!(
                f64_near(points[i].x(), test_point.x(), f64::EPSILON * 10.0),
                "{} (Actual) vs {} (Expected)",
                points[i].x(),
                test_point.x()
            );
            assert!(
                f64_near(points[i].y(), test_point.y(), f64::EPSILON * 10.0),
                "{} (Actual) vs {} (Expected)",
                points[i].y(),
                test_point.y()
            );
        }
    }

    #[test]
    fn test_3d_construct() {
        let bases = Vec::<Point3D>::new();
        let curve = PowerBasisCurve3D::from_bases(bases);
        let point = curve.evaluate(0.5);
        assert!(f64_equal(point.x(), 0.0));
        assert!(f64_equal(point.y(), 0.0));
        assert!(f64_equal(point.z(), 0.0));
    }

    #[test]
    fn test_3d_point() {
        let mut bases = Vec::<Point3D>::with_capacity(3);
        bases.push(Point3D::new([0.0, 0.0, 0.0]));
        bases.push(Point3D::new([1.0, 2.0, 2.0]));
        bases.push(Point3D::new([2.0, 0.0, 1.0]));
        let curve = PowerBasisCurve3D::from_bases(bases);
        let point = curve.evaluate(0.5);
        assert!(f64_equal(point.x(), 1.0));
        assert!(f64_equal(point.y(), 1.0));
        assert!(f64_equal(point.z(), 1.25));
    }

    #[test]
    fn test_3d_points() {
        let mut bases = Vec::<Point3D>::with_capacity(3);
        bases.push(Point3D::new([0.0, 0.0, 0.0]));
        bases.push(Point3D::new([1.0, 2.0, 2.0]));
        bases.push(Point3D::new([2.0, 0.0, 1.0]));
        let curve = PowerBasisCurve3D::from_bases(bases);

        let points = curve.evaluate_points(100);

        let p0 = Point3D::new([0.0, 0.0, 0.0]);
        let p1 = Point3D::new([1.0, 2.0, 2.0]);
        let p2 = Point3D::new([2.0, 0.0, 1.0]);
        let div = 1.0 / 99.0;
        for i in 0..100 {
            let param = i as f64 * div;
            let test_point = p0 + (p1 * param) + (p2 * param * param);

            assert!(
                f64_near(points[i].x(), test_point.x(), f64::EPSILON * 10.0),
                "{} (Actual) vs {} (Expected)",
                points[i].x(),
                test_point.x()
            );
            assert!(
                f64_near(points[i].y(), test_point.y(), f64::EPSILON * 10.0),
                "{} (Actual) vs {} (Expected)",
                points[i].y(),
                test_point.y()
            );
            assert!(
                f64_near(points[i].z(), test_point.z(), f64::EPSILON * 10.0),
                "{} (Actual) vs {} (Expected)",
                points[i].z(),
                test_point.z()
            );
        }
    }
}
