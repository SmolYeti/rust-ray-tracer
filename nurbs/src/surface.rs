use crate::interval::Interval;
use crate::point_types::Point2D;
use crate::point_types::Point3D;

pub trait Surface {
    fn interval_u(&self) -> &Interval;
    fn interval_v(&self) -> &Interval;

    fn evaluate(&self, uv: Point2D) -> Point3D;

    fn evaluate_points(&self, u_sample_count: usize, v_sample_count: usize) -> Vec<Point3D> {
        let mut points = Vec::<Point3D>::with_capacity(u_sample_count * v_sample_count);
        let u_div = (self.interval_u().max() - self.interval_u().min()) / ((u_sample_count as f64) - 1.0);
        let v_div = (self.interval_v().max() - self.interval_v().min()) / ((u_sample_count as f64) - 1.0);

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