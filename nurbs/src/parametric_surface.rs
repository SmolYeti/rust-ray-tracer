use crate::interval::Interval;
use crate::point_types::Point2D;
use crate::point_types::Point3D;
use crate::surface::Surface;

pub struct ParametricSurface {
    functions: [Box<dyn Fn(Point2D) -> f64>; 3],
    u_interval: Interval,
    v_interval: Interval,
}

impl ParametricSurface {
    pub fn new(
        functions: [Box<dyn Fn(Point2D) -> f64>; 3],
        u_interval: Interval,
        v_interval: Interval,
    ) -> ParametricSurface {
        ParametricSurface {
            functions,
            u_interval,
            v_interval,
        }
    }

    pub fn from_functions(functions: [Box<dyn Fn(Point2D) -> f64>; 3]) -> ParametricSurface {
        let interval = Interval::from_values(0.0, 1.0);
        ParametricSurface::new(functions, interval.clone(), interval)
    }
}

impl Surface for ParametricSurface {
    fn interval_u(&self) -> &Interval {
        &self.u_interval
    }

    fn interval_v(&self) -> &Interval {
        &self.v_interval
    }

    fn evaluate(&self, uv: Point2D) -> Point3D {
        let parameter = Point2D::new([
            self.interval_u().clamp_value(uv.u()),
            self.interval_v().clamp_value(uv.v()),
        ]);

        Point3D::new([
            self.functions[0](parameter),
            self.functions[1](parameter),
            self.functions[2](parameter),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use crate::parametric_surface::ParametricSurface;
    use crate::point_types::Point2D;
    use crate::surface::Surface;
    use crate::utility::f64_equal;
    use crate::utility::f64_near;
    use core::f64;

    #[test]
    fn test_construct() {
        let functions: [Box<dyn Fn(Point2D) -> f64 + 'static>; 3] = [
            Box::new(|uv: Point2D| uv.u().sin() * uv.v().cos()),
            Box::new(|uv: Point2D| uv.x().sin() * uv.y().sin()),
            Box::new(|uv: Point2D| uv.x().cos()),
        ];
        let interval = Interval::from_values(0.0, f64::consts::PI * 2.0);
        let surface = ParametricSurface::new(functions, interval.clone(), interval);

        assert!(f64_equal(surface.interval_u().min(), 0.0));
        assert!(f64_equal(surface.interval_u().max(), f64::consts::PI * 2.0));
        assert!(f64_equal(surface.interval_v().min(), 0.0));
        assert!(f64_equal(surface.interval_v().max(), f64::consts::PI * 2.0));
    }

    #[test]
    fn test_point() {
        let point_count = 100;
        let div = f64::consts::PI * 2.0 / ((point_count - 1) as f64);
        let functions: [Box<dyn Fn(Point2D) -> f64 + 'static>; 3] = [
            Box::new(|uv: Point2D| uv.u().sin() * uv.v().cos()),
            Box::new(|uv: Point2D| uv.x().sin() * uv.y().sin()),
            Box::new(|uv: Point2D| uv.x().cos()),
        ];
        let interval = Interval::from_values(0.0, f64::consts::PI * 2.0);
        let surface = ParametricSurface::new(functions, interval.clone(), interval);

        let points = surface.evaluate_points(point_count, point_count);

        for i in 0..point_count {
            let u = i as f64 * div;
            for j in 0..point_count {
                let v = j as f64 * div;
                let point = points[i * point_count + j];
                assert!(f64_equal(point.x(), u.sin() * v.cos()));
                assert!(f64_equal(point.y(), u.sin() * v.sin()));
                assert!(f64_equal(point.z(), u.cos()));

                let length = (point.x().powi(2) + point.y().powi(2) + point.z().powi(2)).sqrt();
                assert!(f64_near(length, 1.0, f64::EPSILON * 10.0), "Actual {} vs 1.0", length);
            }
        }
    }
}
