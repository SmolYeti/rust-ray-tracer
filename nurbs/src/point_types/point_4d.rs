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

    #[test]
    fn test_new() {
        let point = Point4D::new([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);
        assert_eq!(point.w(), 4.0);
    }

    #[test]
    fn test_empty() {
    let point = Point4D::empty();
        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 0.0);
        assert_eq!(point.z(), 0.0);
        assert_eq!(point.w(), 0.0);
    }
}
