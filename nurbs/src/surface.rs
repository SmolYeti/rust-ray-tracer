use crate::interval::Interval;
use crate::point_types::Point2D;
use crate::point_types::Point3D;

pub trait Surface {
    fn interval_u(&self) -> &Interval;
    fn interval_v(&self) -> &Interval;

    fn evaluate(&self, uv: Point2D) -> Point3D;

    fn evaluate_points(&self, u_sample_count: u32, v_sample_count: u32) -> vec<Point3D> {
        let points = vec<Point3D>::with_capacity(u_sample_count * v_sample_count);
        points
    }
    /*virtual std::vector<Point3D> EvaluatePoints(uint32_t u_sample_count,
                                              uint32_t v_sample_count) const {
    std::vector<Point3D> points(u_sample_count * v_sample_count);
    double u_div = (u_interval_.y - u_interval_.x) /
                   static_cast<double>(u_sample_count - 1);
    double v_div = (v_interval_.y - v_interval_.x) /
                   static_cast<double>(v_sample_count - 1);
    for (uint32_t i = 0; i < u_sample_count; ++i) {
      Point2D uv = {u_interval_.x + static_cast<double>(i) * u_div, 0};
      for (uint32_t j = 0; j < v_sample_count; ++j) {
        uv.y = v_interval_.x + static_cast<double>(j) * v_div;
        points[(i * v_sample_count) + j] = EvaluatePoint(uv);
      }
    }
    return points;
  }*/
}