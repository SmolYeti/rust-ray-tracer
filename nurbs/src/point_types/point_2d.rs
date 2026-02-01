use crate::point_types::Point2D;

impl Point2D {
    pub fn x(self) -> f64 {
        self.values[0]
    }
    
    pub fn y(self) -> f64 {
        self.values[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::point_types::Point2D;

    #[test]
    fn test_new() {
        let point = Point2D::new([1.0, 2.0]);
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
    }

    #[test]
    fn test_empty() {
        let point = Point2D::empty();
        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 0.0);
    }
}
