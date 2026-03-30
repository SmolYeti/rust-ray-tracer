use crate::bezier_curve::BezierCurve3D;
use crate::curve::Curve3D;
use crate::interval::Interval;
use crate::point_types::Point2D;
use crate::point_types::Point3D;
use crate::surface::Surface;

pub struct BezierSurface {
    curves: Vec<BezierCurve3D>,
    u_interval: Interval,
    v_interval: Interval,
}

impl BezierSurface {
     pub fn new(curves: Vec<BezierCurve3D>, u_interval: Interval, v_interval: Interval) -> BezierSurface {
        BezierSurface {
            curves,
            u_interval,
            v_interval,
        }
    }

    pub fn from_curves(curves: Vec<BezierCurve3D>) -> BezierSurface {
        let interval = Interval::new(Point2D::new([0.0, 1.0]));
        BezierSurface::new(curves, interval.clone(), interval)
    }

    pub fn from_points(points: Vec<Point3D>, curve_length: usize, u_interval: Interval, v_interval: Interval) -> BezierSurface {
        let curve_count = points.len() / curve_length;
        let mut curves: Vec<BezierCurve3D> = Vec::with_capacity(curve_count);

        for curve in 0..curve_count {
            let mut control_points = Vec::with_capacity(curve_length);
            for point in 0..curve_length {
                let index = curve * curve_length + point;
                control_points.push(points[index]);
            }
            curves.push(BezierCurve3D::new(control_points, u_interval));
        }
        
        BezierSurface { curves, u_interval, v_interval }
    }
}

impl Surface for BezierSurface {
    fn interval_u(&self) -> &Interval {
        &self.u_interval
    }

    fn interval_v(&self) -> &Interval{
        &self.v_interval
    }

    fn evaluate(&self, uv: Point2D) -> Point3D {
        let mut control_points = Vec::with_capacity(self.curves.len());
        for curve in 0..self.curves.len() {
            control_points.push(self.curves[curve].evaluate(uv.x()));
        }
        let bezier_curve = BezierCurve3D::new(control_points, self.interval_v().clone());
        bezier_curve.evaluate(uv.y())
    }

    fn evaluate_points(&self, u_sample_count: usize, v_sample_count: usize) -> Vec<Point3D> {
        let mut points = Vec::<Point3D>::with_capacity(u_sample_count * v_sample_count);
        let u_div =
            (self.interval_u().max() - self.interval_u().min()) / ((u_sample_count as f64) - 1.0);
        let v_div =
            (self.interval_v().max() - self.interval_v().min()) / ((v_sample_count as f64) - 1.0);

        for i in 0..u_sample_count {
            let u = self.interval_u().min() + (i as f64 * u_div);
            let mut control_points = Vec::with_capacity(self.curves.len());
            for curve in 0..self.curves.len() {
                control_points.push(self.curves[curve].evaluate(u));
            }
            let bezier_curve = BezierCurve3D::new(control_points, self.interval_v().clone());

            for j in 0..v_sample_count {
                let v = self.interval_v().min() + (j as f64 * v_div);
                points.push(bezier_curve.evaluate(v));
            }
        }

        points
    }
}

#[cfg(test)]
mod tests {}
/*

TEST(NURBS_Chapter1, BezierSurfaceConstruct) {
  const std::vector<BezierCurve3D> curves;
  const BezierSurface surface(curves);
}

TEST(NURBS_Chapter1, BezierSurfacePoint) {
  std::vector<BezierCurve3D> curves;
  {
    std::vector<Point3D> control_points;
    control_points.push_back({0, -1, 0});
    control_points.push_back({0, 2, 1});
    control_points.push_back({0, 2, 2});
    control_points.push_back({0, 1, 3});
    curves.push_back(control_points);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({1, 0, 0});
    control_points.push_back({1, 4, 1});
    control_points.push_back({1, 3, 2});
    control_points.push_back({1, 2, 3});
    curves.push_back(control_points);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({2, 2, 0});
    control_points.push_back({2, 1, 1});
    control_points.push_back({2, 0, 2});
    control_points.push_back({2, -1, 3});
    curves.push_back(control_points);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({3, 3, 0});
    control_points.push_back({3, -2, 1});
    control_points.push_back({3, -4, 2});
    control_points.push_back({3, 0, 3});
    curves.push_back(control_points);
  }
  const BezierSurface surface(curves);

  const Point3D point = surface.EvaluatePoint({0.5, 0.5});

  const std::vector<Point3D> temp_cps = {
      curves[0].EvaluateCurve(0.5), curves[1].EvaluateCurve(0.5),
      curves[2].EvaluateCurve(0.5), curves[3].EvaluateCurve(0.5)};
  const BezierCurve3D curve(temp_cps);
  const Point3D test_point = curve.EvaluateCurve(0.5);

  EXPECT_DOUBLE_EQ(test_point.x, point.x);
  EXPECT_DOUBLE_EQ(test_point.y, point.y);
  EXPECT_DOUBLE_EQ(test_point.z, point.z);
}

TEST(NURBS_Chapter1, BezierSurfacePointInterval) {
  Point2D u_interval = {0.0, 10.0};
  Point2D v_interval = {-12.0, -10.0};
  double u_mid = (u_interval.x + u_interval.y) * 0.5;
  double v_mid = (v_interval.x + v_interval.y) * 0.5;
  std::vector<BezierCurve3D> curves;
  {
    std::vector<Point3D> control_points;
    control_points.push_back({0, -1, 0});
    control_points.push_back({0, 2, 1});
    control_points.push_back({0, 2, 2});
    control_points.push_back({0, 1, 3});
    curves.emplace_back(control_points, u_interval);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({1, 0, 0});
    control_points.push_back({1, 4, 1});
    control_points.push_back({1, 3, 2});
    control_points.push_back({1, 2, 3});
    curves.emplace_back(control_points, u_interval);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({2, 2, 0});
    control_points.push_back({2, 1, 1});
    control_points.push_back({2, 0, 2});
    control_points.push_back({2, -1, 3});
    curves.emplace_back(control_points, u_interval);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({3, 3, 0});
    control_points.push_back({3, -2, 1});
    control_points.push_back({3, -4, 2});
    control_points.push_back({3, 0, 3});
    curves.emplace_back(control_points, u_interval);
  }
  const BezierSurface surface(curves, u_interval, v_interval);

  const Point3D point = surface.EvaluatePoint({u_mid, v_mid});

  const std::vector<Point3D> temp_cps = {
      curves[0].EvaluateCurve(u_mid), curves[1].EvaluateCurve(u_mid),
      curves[2].EvaluateCurve(u_mid), curves[3].EvaluateCurve(u_mid)};
  const BezierCurve3D curve(temp_cps, v_interval);
  const Point3D test_point = curve.EvaluateCurve(v_mid);

  EXPECT_DOUBLE_EQ(test_point.x, point.x);
  EXPECT_DOUBLE_EQ(test_point.y, point.y);
  EXPECT_DOUBLE_EQ(test_point.z, point.z);
}

TEST(NURBS_Chapter1, BezierSurfacePoints) {
  constexpr uint32_t point_count = 100;
  constexpr double div = 1.0 / static_cast<double>(point_count - 1);
  std::vector<BezierCurve3D> curves;
  {
    std::vector<Point3D> control_points;
    control_points.push_back({0, -1, 0});
    control_points.push_back({0, 2, 1});
    control_points.push_back({0, 2, 2});
    control_points.push_back({0, 1, 3});
    curves.push_back(control_points);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({1, 0, 0});
    control_points.push_back({1, 4, 1});
    control_points.push_back({1, 3, 2});
    control_points.push_back({1, 2, 3});
    curves.push_back(control_points);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({2, 2, 0});
    control_points.push_back({2, 1, 1});
    control_points.push_back({2, 0, 2});
    control_points.push_back({2, -1, 3});
    curves.push_back(control_points);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({3, 3, 0});
    control_points.push_back({3, -2, 1});
    control_points.push_back({3, -4, 2});
    control_points.push_back({3, 0, 3});
    curves.push_back(control_points);
  }
  const BezierSurface surface(curves);

  const std::vector<Point3D> points =
      surface.EvaluatePoints(point_count, point_count);

  for (uint32_t i = 0; i < point_count; ++i) {
    const double u = static_cast<double>(i) * div;
    const std::vector<Point3D> temp_cps = {
        curves[0].EvaluateCurve(u), curves[1].EvaluateCurve(u),
        curves[2].EvaluateCurve(u), curves[3].EvaluateCurve(u)};
    const BezierCurve3D curve(temp_cps);
    for (uint32_t j = 0; j < point_count; ++j) {
      const double v = static_cast<double>(j) * div;
      const Point3D test_point = curve.EvaluateCurve(v);
      const Point3D &point = points[(i * point_count) + j];

      EXPECT_DOUBLE_EQ(test_point.x, point.x);
      EXPECT_DOUBLE_EQ(test_point.y, point.y);
      EXPECT_DOUBLE_EQ(test_point.z, point.z);
    }
  }
}

TEST(NURBS_Chapter1, BezierSurfacePointsInterval) {
  constexpr double tolerance = std::numeric_limits<double>::epsilon() * 100.0;
  const Point2D u_interval = {-10.0, 10.0};
  const Point2D v_interval = {22.0, 30.0};
  const double u_mid = (u_interval.x + u_interval.y) * 0.5;
  const double v_mid = (v_interval.x + v_interval.y) * 0.5;
  constexpr uint32_t point_count = 100;
  constexpr double div = 1.0 / static_cast<double>(point_count - 1);
  const double u_div = (div * (u_interval.y - u_interval.x));
  const double v_div = (div * (v_interval.y - v_interval.x));
  std::vector<BezierCurve3D> curves;
  {
    std::vector<Point3D> control_points;
    control_points.push_back({0, -1, 0});
    control_points.push_back({0, 2, 1});
    control_points.push_back({0, 2, 2});
    control_points.push_back({0, 1, 3});
    curves.emplace_back(control_points, u_interval);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({1, 0, 0});
    control_points.push_back({1, 4, 1});
    control_points.push_back({1, 3, 2});
    control_points.push_back({1, 2, 3});
    curves.emplace_back(control_points, u_interval);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({2, 2, 0});
    control_points.push_back({2, 1, 1});
    control_points.push_back({2, 0, 2});
    control_points.push_back({2, -1, 3});
    curves.emplace_back(control_points, u_interval);
  }
  {
    std::vector<Point3D> control_points;
    control_points.push_back({3, 3, 0});
    control_points.push_back({3, -2, 1});
    control_points.push_back({3, -4, 2});
    control_points.push_back({3, 0, 3});
    curves.emplace_back(control_points, u_interval);
  }
  const BezierSurface surface(curves, u_interval, v_interval);

  const std::vector<Point3D> points =
      surface.EvaluatePoints(point_count, point_count);

  for (uint32_t i = 0; i < point_count; ++i) {
    const double u = (static_cast<double>(i) * u_div) + u_interval.x;
    const std::vector<Point3D> temp_cps = {
        curves[0].EvaluateCurve(u), curves[1].EvaluateCurve(u),
        curves[2].EvaluateCurve(u), curves[3].EvaluateCurve(u)};
    const BezierCurve3D curve(temp_cps, v_interval);
    for (uint32_t j = 0; j < point_count; ++j) {
      const double v = (static_cast<double>(j) * v_div) + v_interval.x;
      const Point3D test_point = curve.EvaluateCurve(v);
      const Point3D &point = points[(i * point_count) + j];

      EXPECT_NEAR(test_point.x, point.x, tolerance);
      EXPECT_NEAR(test_point.y, point.y, tolerance);
      EXPECT_NEAR(test_point.z, point.z, tolerance);
    }
  }
}
*/
