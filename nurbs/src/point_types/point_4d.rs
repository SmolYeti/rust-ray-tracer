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

    // Construct
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

    // Dot
    #[test]
    fn test_dot() {
        let point_0 = Point4D::new([1.0, 1.5, 2.0, 2.5]);
        let point_1 = Point4D::new([0.4, 3.0, 1.2, 4.0]);
        let test_val = point_0.dot(point_1);
        assert!(f64_equal(test_val, 17.3), "Value was: {}", test_val);
    }

    // Addition
    #[test]
    fn test_add_point_f64() {
        let point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        let test_point = point + val;
        assert!(f64_equal(test_point.x(), 2.3));
        assert!(f64_equal(test_point.y(), 3.1));
        assert!(f64_equal(test_point.z(), 6.4));
        assert!(f64_equal(test_point.w(), 7.6));
    }

    #[test]
    fn test_add_f64_point() {
        let point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        let test_point = val + point;
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
    }

    // Subtraction
    #[test]
    fn test_sub_point_f64() {
        let point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        let test_point = point - val;
        assert!(f64_equal(test_point.x(), -1.7), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), -0.9), "Value was: {}", test_point.y());
        assert!(f64_near(test_point.z(), 2.4, f64::EPSILON * 10.0), "Value was: {}", test_point.z());
        assert!(f64_near(test_point.w(), 3.6, f64::EPSILON * 10.0), "Value was: {}", test_point.w());
    }

    #[test]
    fn test_sub_f64_point() {
        let point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        let test_point = val - point;
        assert!(f64_equal(test_point.x(), 1.7), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 0.9), "Value was: {}", test_point.y());
        assert!(f64_near(test_point.z(), -2.4, f64::EPSILON * 10.0), "Value was: {}", test_point.z());
        assert!(f64_near(test_point.w(), -3.6, f64::EPSILON * 10.0), "Value was: {}", test_point.w());
    }

    #[test]
    fn test_sub_point() {
        let point_0 = Point4D::new([0.3, 1.1, 2.5, 5.6]);
        let point_1 = Point4D::new([0.4, 5.2, 1.2, 3.9]);
        let test_point = point_0 - point_1;
        assert!(f64_equal(test_point.x(), -0.1), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), -4.1), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 1.3), "Value was: {}", test_point.z());
        assert!(f64_near(test_point.w(), 1.7, f64::EPSILON * 10.0), "Value was: {}", test_point.w());
    }

    #[test]
    fn test_sub_equals_f64() {
        let mut test_point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        test_point -= val;
        assert!(f64_equal(test_point.x(), -1.7), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), -0.9), "Value was: {}", test_point.y());
        assert!(f64_near(test_point.z(), 2.4, f64::EPSILON * 10.0), "Value was: {}", test_point.z());
        assert!(f64_near(test_point.w(), 3.6, f64::EPSILON * 10.0), "Value was: {}", test_point.w());
    }

    #[test]
    fn test_sub_equals_point() {
        let mut test_point = Point4D::new([0.3, 1.1, 2.5, 5.6]);
        let point = Point4D::new([0.4, 5.2, 1.2, 3.9]);
        test_point -= point;
        assert!(f64_equal(test_point.x(), -0.1), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), -4.1), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 1.3), "Value was: {}", test_point.z());
        assert!(f64_near(test_point.w(), 1.7, f64::EPSILON * 10.0), "Value was: {}", test_point.w());
    }

    // Multiplication
    #[test]
    fn test_mul_point_f64() {
        let point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        let test_point = point * val;
        assert!(f64_equal(test_point.x(), 0.6), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 2.2), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 8.8), "Value was: {}", test_point.z());
        assert!(f64_equal(test_point.w(), 11.2), "Value was: {}", test_point.w());
    }
    
    #[test]
    fn test_mul_f64_point() {
        let point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        let test_point = val * point;
        assert!(f64_equal(test_point.x(), 0.6), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 2.2), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 8.8), "Value was: {}", test_point.z());
        assert!(f64_equal(test_point.w(), 11.2), "Value was: {}", test_point.w());
    }

    #[test]
    fn test_mul_equals_f64() {
        let mut test_point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        test_point *= val;
        assert!(f64_equal(test_point.x(), 0.6), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 2.2), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 8.8), "Value was: {}", test_point.z());
        assert!(f64_equal(test_point.w(), 11.2), "Value was: {}", test_point.w());
    }

    // Division
    #[test]
    fn test_div_point_f64() {
        let point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        let test_point = point / val;
        assert!(f64_equal(test_point.x(), 0.15), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 0.55), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 2.2), "Value was: {}", test_point.z());
        assert!(f64_equal(test_point.w(), 2.8), "Value was: {}", test_point.w());
    }
    
    #[test]
    fn test_div_f64_point() {
        let point = Point4D::new([2.0, 4.0, 8.0, 16.0]);
        let val = 2.0;
        let test_point = val / point;
        assert!(f64_equal(test_point.x(), 1.0), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 0.5), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 0.25), "Value was: {}", test_point.z());
        assert!(f64_equal(test_point.w(), 0.125), "Value was: {}", test_point.w());
    }

    #[test]
    fn test_div_equals_f64() {
        let mut test_point = Point4D::new([0.3, 1.1, 4.4, 5.6]);
        let val = 2.0;
        test_point /= val;
        assert!(f64_equal(test_point.x(), 0.15), "Value was: {}", test_point.x());
        assert!(f64_equal(test_point.y(), 0.55), "Value was: {}", test_point.y());
        assert!(f64_equal(test_point.z(), 2.2), "Value was: {}", test_point.z());
        assert!(f64_equal(test_point.w(), 2.8), "Value was: {}", test_point.w());
    }
}
