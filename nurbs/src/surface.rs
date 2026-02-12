use crate::interval::Interval;
use crate::point_types::Point2D;
use crate::point_types::Point3D;

pub trait Surface {
    fn interval_u(&self) -> &Interval;
    fn interval_v(&self) -> &Interval;

    fn evaluate(&self, uv: Point2D) -> Point3D;

    fn evaluate_points(&self, u_sample_count: usize, v_sample_count: usize) -> Vec<Point3D> {
        let mut points = Vec::<Point3D>::with_capacity(u_sample_count * v_sample_count);
        let u_div =
            (self.interval_u().max() - self.interval_u().min()) / ((u_sample_count as f64) - 1.0);
        let v_div =
            (self.interval_v().max() - self.interval_v().min()) / ((v_sample_count as f64) - 1.0);

        for i in 0..u_sample_count {
            let u = self.interval_u().min() + (i as f64 * u_div);
            for j in 0..v_sample_count {
                let v = self.interval_v().min() + (j as f64 * v_div);
                points.push(self.evaluate(Point2D::new([u, v])));
            }
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use crate::point_types::Point2D;
    use crate::point_types::Point3D;
    use crate::surface::Surface;
    use crate::utility::f64_equal;

    pub struct ImplSurface {
        u_interval: Interval,
        v_interval: Interval,
    }

    impl Surface for ImplSurface {
        fn interval_u(&self) -> &Interval {
            &self.u_interval
        }

        fn interval_v(&self) -> &Interval {
            &self.v_interval
        }

        fn evaluate(&self, uv: Point2D) -> Point3D {
            let param_u = self.u_interval.clamp_value(uv.u());
            let param_v = self.v_interval.clamp_value(uv.v());
            Point3D::new([param_u, param_v, 1.0])
        }
    }

    impl ImplSurface {
        pub fn new(u_interval: Interval, v_interval: Interval) -> ImplSurface {
            ImplSurface {
                u_interval,
                v_interval,
            }
        }
    }

    #[test]
    fn test_surface() {
        let u_interval = Interval::new(Point2D::new([0.0, 1.0]));
        let v_interval = Interval::new(Point2D::new([2.0, 3.0]));
        let surface = ImplSurface::new(u_interval, v_interval);

        let interval_u = surface.interval_u();
        assert!(f64_equal(interval_u.min(), 0.0));
        assert!(f64_equal(interval_u.max(), 1.0));

        let interval_v = surface.interval_v();
        assert!(f64_equal(interval_v.min(), 2.0));
        assert!(f64_equal(interval_v.max(), 3.0));

        let uv = Point2D::new([1.5, 1.5]);
        let eval = surface.evaluate(uv);
        assert!(f64_equal(eval.x(), 1.0));
        assert!(f64_equal(eval.y(), 2.0));
        assert!(f64_equal(eval.z(), 1.0));

        let points = surface.evaluate_points(5, 10);

        assert_eq!(points.len(), 50);
        for i in 0..5 {
            let test_u_val = i as f64 * (1.0 / 4.0);
            for j in 0..10 {
                let test_v_val = 2.0 + (j as f64 * (1.0 / 9.0));
                let point = points[(i * 10) + j];
                assert!(
                    f64_equal(point.x(), test_u_val),
                    "X value was {} vs {}",
                    point.x(),
                    test_u_val
                );
                assert!(
                    f64_equal(point.y(), test_v_val),
                    "Y value was {} vs {}",
                    point.y(),
                    test_v_val
                );
                assert!(
                    f64_equal(point.z(), 1.0),
                    "Z value was {} vs {}",
                    point.z(),
                    1.0
                );
            }
        }
    }
}
