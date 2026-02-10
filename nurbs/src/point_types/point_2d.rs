use crate::point_types::Point2D;

impl Point2D {
    pub fn x(&self) -> f64 {
        self.values[0]
    }
    
    pub fn y(&self) -> f64 {
        self.values[1]
    }

    pub fn u(&self) -> f64 {
        self.values[0]
    }
    
    pub fn v(&self) -> f64 {
        self.values[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::point_types::Point2D;
    use crate::utility::f64_equal;
    use crate::utility::f64_near;

    // Construct
    #[test]
    fn test_new() {
        let point = Point2D::new([1.0, 2.0]);
        assert!(f64_equal(point.x(), 1.0));
        assert!(f64_equal(point.y(), 2.0));
        assert!(f64_equal(point.u(), 1.0));
        assert!(f64_equal(point.v(), 2.0));
    }

    #[test]
    fn test_empty() {
        let point = Point2D::empty();
        assert!(f64_equal(point.x(), 0.0));
        assert!(f64_equal(point.y(), 0.0));
        assert!(f64_equal(point.u(), 0.0));
        assert!(f64_equal(point.v(), 0.0));
    }

    // Dot
    #[test]
    fn test_dot() {
        let point_0 = Point2D::new([1.0, 1.5]);
        let point_1 = Point2D::new([0.4, 3.0]);
        let test_val = point_0.dot(point_1);
        assert!(f64_equal(test_val, 4.9), "Value was: {}", test_val);
    }

    // Addition
    #[test]
    fn test_add_point_f64() {
        let point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        let test_point = point + val;
        assert!(f64_equal(test_point.x(), 2.3));
        assert!(f64_equal(test_point.y(), 3.1));
    }

    #[test]
    fn test_add_f64_point() {
        let point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        let test_point = val + point;
        assert!(f64_equal(test_point.x(), 2.3));
        assert!(f64_equal(test_point.y(), 3.1));
    }

    #[test]
    fn test_add_point() {
        let point_0 = Point2D::new([0.3, 1.1]);
        let point_1 = Point2D::new([0.4, 5.2]);
        let test_point = point_0 + point_1;
        assert!(f64_equal(test_point.x(), 0.7), "Value was: {}", test_point.x());
        assert!(f64_near(test_point.y(), 6.3, f64::EPSILON * 10.0), "Value was: {}", test_point.y());
    }

    #[test]
    fn test_add_equals_f64() {
        let mut point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        point += val;
        assert!(f64_equal(point.x(), 2.3));
        assert!(f64_equal(point.y(), 3.1));
    }
    
    #[test]
    fn test_add_equals_point() {
        let mut test_point = Point2D::new([0.3, 1.1]);
        let point_1 = Point2D::new([0.4, 5.2]);
        test_point += point_1;
        assert!(f64_equal(test_point.x(), 0.7), "Value was: {}", test_point.x());
        assert!(f64_near(test_point.y(), 6.3, f64::EPSILON * 10.0), "Value was: {}", test_point.y());
    }

    // Subtraction
    #[test]
    fn test_sub_point_f64() {
        let point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        let test_point = point - val;
        assert!(f64_equal(test_point.x(), -1.7), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), -0.9), "Value was: {}", test_point.y());
    }

    #[test]
    fn test_sub_f64_point() {
        let point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        let test_point = val - point;
        assert!(f64_equal(test_point.x(), 1.7), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 0.9), "Value was: {}", test_point.y());
    }

    #[test]
    fn test_sub_point() {
        let point_0 = Point2D::new([0.3, 1.1]);
        let point_1 = Point2D::new([0.4, 5.2]);
        let test_point = point_0 - point_1;
        assert!(f64_equal(test_point.x(), -0.1), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), -4.1), "Value was: {}", test_point.y());
    }

    #[test]
    fn test_sub_equals_f64() {
        let mut test_point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        test_point -= val;
        assert!(f64_equal(test_point.x(), -1.7), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), -0.9), "Value was: {}", test_point.y());
    }

    #[test]
    fn test_sub_equals_point() {
        let mut test_point = Point2D::new([0.3, 1.1]);
        let point = Point2D::new([0.4, 5.2]);
        test_point -= point;
        assert!(f64_equal(test_point.x(), -0.1), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), -4.1), "Value was: {}", test_point.y());
    }

    // Multiplication
    #[test]
    fn test_mul_point_f64() {
        let point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        let test_point = point * val;
        assert!(f64_equal(test_point.x(), 0.6), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 2.2), "Value was: {}", test_point.y());
    }
    
    #[test]
    fn test_mul_f64_point() {
        let point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        let test_point = val * point;
        assert!(f64_equal(test_point.x(), 0.6), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 2.2), "Value was: {}", test_point.y());
    }

    #[test]
    fn test_mul_equals_f64() {
        let mut test_point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        test_point *= val;
        assert!(f64_equal(test_point.x(), 0.6), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 2.2), "Value was: {}", test_point.y());
    }

    // Division
    #[test]
    fn test_div_point_f64() {
        let point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        let test_point = point / val;
        assert!(f64_equal(test_point.x(), 0.15), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 0.55), "Value was: {}", test_point.y());
    }
    
    #[test]
    fn test_div_f64_point() {
        let point = Point2D::new([2.0, 4.0]);
        let val = 2.0;
        let test_point = val / point;
        assert!(f64_equal(test_point.x(), 1.0), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 0.5), "Value was: {}", test_point.y());
    }

    #[test]
    fn test_div_equals_f64() {
        let mut test_point = Point2D::new([0.3, 1.1]);
        let val = 2.0;
        test_point /= val;
        assert!(f64_equal(test_point.x(), 0.15), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 0.55), "Value was: {}", test_point.y());
    }
}
