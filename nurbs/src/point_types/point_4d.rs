use crate::point_types::Point4D;

impl Point4D {
    pub fn x(self) -> f64 {
        self.values[0]
    }
    
    pub fn y(self) -> f64 {
        self.values[1]
    }

    pub fn z(self) -> f64 {
        self.values[2]
    }

    pub fn w(self) -> f64 {
        self.values[3]
    }
}

#[cfg(test)]
mod tests {
    use crate::point_types::Point4D;
    use crate::utility::f64_equal;
    use crate::utility::f64_near;

    #[test]
    fn test_new() {
        let point = Point4D::new([1.0, 2.0, 3.0, 4.0]);
        assert!(f64_equal(point.x(), 1.0));
        assert!(f64_equal(point.y(), 2.0));
        assert!(f64_equal(point.z(), 3.0));
        assert!(f64_equal(point.w(), 4.0));
    }

    #[test]
    fn test_empty() {
        let point = Point4D::empty();
        assert!(f64_equal(point.x(), 0.0));
        assert!(f64_equal(point.y(), 0.0));
        assert!(f64_equal(point.z(), 0.0));
        assert!(f64_equal(point.w(), 0.0));
    }

    // Addition
    #[test]
    fn test_add_f64() {
        let point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        let test_point = point + val;
        assert!(f64_equal(test_point.x(), 2.3));
        assert!(f64_equal(test_point.y(), 3.1));
        assert!(f64_equal(test_point.z(), 6.4));
        assert!(f64_equal(test_point.w(), 7.6));
    }

    #[test]
    fn test_add_point() {
        let point_0 = Point4D::new([0.3, 1.1, 2.5, 5.6]);
        let point_1 = Point4D::new([0.4, 5.2, 1.2, 3.9]);
        let test_point = point_0 + point_1;
        assert!(f64_equal(test_point.x(), 0.7), "Value was: {}", test_point.x());
        assert!(f64_near(test_point.y(), 6.3, f64::EPSILON * 10.0), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 3.7), "Value was: {}", test_point.z());
        assert!(f64_equal(test_point.w(), 9.5), "Value was: {}", test_point.w());
    }

    #[test]
    fn test_add_equals_f64() {
        let mut point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        point += val;
        assert!(f64_equal(point.x(), 2.3));
        assert!(f64_equal(point.y(), 3.1));
        assert!(f64_equal(point.z(), 6.4));
        assert!(f64_equal(point.w(), 7.6));
    }
    
    #[test]
    fn test_add_equals_point() {
        let mut test_point = Point4D::new([0.3, 1.1, 2.5, 5.6]);
        let point_1 = Point4D::new([0.4, 5.2, 1.2, 3.9]);
        test_point += point_1;
        assert!(f64_equal(test_point.x(), 0.7), "Value was: {}", test_point.x());
        assert!(f64_near(test_point.y(), 6.3, f64::EPSILON * 10.0), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 3.7), "Value was: {}", test_point.z());
        assert!(f64_equal(test_point.w(), 9.5), "Value was: {}", test_point.w());
    }/*

// Subtraction
TEST(Point4DTest, SubPointDouble) {
  const Point4D point = {0.3, 1.1, 2.21, 5.6};
  constexpr double val = 2.0;
  Point4D test_point = point - val;
  EXPECT_DOUBLE_EQ(test_point.x, -1.7);
  EXPECT_DOUBLE_EQ(test_point.y, -0.9);
  EXPECT_DOUBLE_EQ(test_point.z, 0.21);
  EXPECT_DOUBLE_EQ(test_point.w, 3.6);
}

TEST(Point4DTest, SubDoublePoint) {
  const Point4D point = {0.3, 1.1, 2.21, 5.6};
  constexpr double val = 2.0;
  const Point4D test_point = val - point;
  EXPECT_DOUBLE_EQ(test_point.x, 1.7);
  EXPECT_DOUBLE_EQ(test_point.y, 0.9);
  EXPECT_DOUBLE_EQ(test_point.z, -0.21);
  EXPECT_DOUBLE_EQ(test_point.w, -3.6);
}

TEST(Point4DTest, SubPointPoint) {
  const Point4D point_0 = {0.3, 1.1, 2.2, 5.61};
  const Point4D point_1 = {0.4, 5.2, 3.7, 4.9};
  const Point4D test_point = point_0 - point_1;
  EXPECT_DOUBLE_EQ(test_point.x, -0.1);
  EXPECT_DOUBLE_EQ(test_point.y, -4.1);
  EXPECT_DOUBLE_EQ(test_point.z, -1.5);
  EXPECT_DOUBLE_EQ(test_point.w, 0.71);
}

TEST(Point4DTest, SubEqualsDouble) {
  Point4D point = {0.3, 1.1, 2.21, 5.6};
  constexpr double val = 2.0;
  point -= val;
  EXPECT_DOUBLE_EQ(point.x, -1.7);
  EXPECT_DOUBLE_EQ(point.y, -0.9);
  EXPECT_DOUBLE_EQ(point.z, 0.21);
  EXPECT_DOUBLE_EQ(point.w, 3.6);
}

TEST(Point4DTest, SubEqualsPoint) {
  Point4D point_0 = {0.3, 1.1, 2.2, 5.61};
  const Point4D point_1 = {0.4, 5.2, 3.7, 4.9};
  point_0 -= point_1;
  EXPECT_DOUBLE_EQ(point_0.x, -0.1);
  EXPECT_DOUBLE_EQ(point_0.y, -4.1);
  EXPECT_DOUBLE_EQ(point_0.z, -1.5);
  EXPECT_DOUBLE_EQ(point_0.w, 0.71);
}

// Multiplication
TEST(Point4DTest, MulPointDouble) {
  const Point4D point = {0.3, 1.1, 2.2, 5.6};
  constexpr double val = 2.0;
  Point4D test_point = point * val;
  EXPECT_DOUBLE_EQ(test_point.x, 0.6);
  EXPECT_DOUBLE_EQ(test_point.y, 2.2);
  EXPECT_DOUBLE_EQ(test_point.z, 4.4);
  EXPECT_DOUBLE_EQ(test_point.w, 11.2);
}

TEST(Point4DTest, MulDoublePoint) {
  const Point4D point = {0.3, 1.1, 2.4, 5.6};
  constexpr double val = 2.0;
  const Point4D test_point = val * point;
  EXPECT_DOUBLE_EQ(test_point.x, 0.6);
  EXPECT_DOUBLE_EQ(test_point.y, 2.2);
  EXPECT_DOUBLE_EQ(test_point.z, 4.8);
  EXPECT_DOUBLE_EQ(test_point.w, 11.2);
}

TEST(Point4DTest, MulEqualsDouble) {
  Point4D point = {0.3, 1.1, 2.4, 5.6};
  constexpr double val = 2.0;
  point *= val;
  EXPECT_DOUBLE_EQ(point.x, 0.6);
  EXPECT_DOUBLE_EQ(point.y, 2.2);
  EXPECT_DOUBLE_EQ(point.z, 4.8);
  EXPECT_DOUBLE_EQ(point.w, 11.2);
}

// Division
TEST(Point4DTest, DivPointDouble) {
  const Point4D point = {0.3, 1.1, 2.4, 5.6};
  constexpr double val = 2.0;
  Point4D test_point = point / val;
  EXPECT_DOUBLE_EQ(test_point.x, 0.15);
  EXPECT_DOUBLE_EQ(test_point.y, 0.55);
  EXPECT_DOUBLE_EQ(test_point.z, 1.2);
  EXPECT_DOUBLE_EQ(test_point.w, 2.8);
}

TEST(Point4DTest, DivDoublePoint) {
  const Point4D point = {0.5, 2.0, 4.0, 8.0};
  constexpr double val = 2.0;
  const Point4D test_point = val / point;
  EXPECT_DOUBLE_EQ(test_point.x, 4.0);
  EXPECT_DOUBLE_EQ(test_point.y, 1.0);
  EXPECT_DOUBLE_EQ(test_point.z, 0.5);
  EXPECT_DOUBLE_EQ(test_point.w, 0.25);
}

TEST(Point4DTest, DivEqualsDouble) {
  Point4D point = {0.3, 1.1, 3.5, 5.6};
  constexpr double val = 2.0;
  point /= val;
  EXPECT_DOUBLE_EQ(point.x, 0.15);
  EXPECT_DOUBLE_EQ(point.y, 0.55);
  EXPECT_DOUBLE_EQ(point.z, 1.75);
  EXPECT_DOUBLE_EQ(point.w, 2.8);
}*/
}
