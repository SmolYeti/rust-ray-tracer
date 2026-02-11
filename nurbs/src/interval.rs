use crate::point_types::Point2D;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    range: Point2D,
    interval_div: f64,
}


impl Interval {
    pub fn new(range: Point2D) -> Interval{
        let mut span = range.y() - range.x();
        if span.abs() < f64::EPSILON {
            span = -1.0
        }
        Interval { range, interval_div : (1.0 / span)}
    }

    pub fn empty() -> Interval {
        Interval::new(Point2D::new([1.0, -1.0]))
    }

    pub fn min(&self) -> f64 {
        self.range.x()
    }

    pub fn max(&self) -> f64 {
        self.range.y()
    }

    pub fn is_valid(&self) -> bool {
        self.interval_div > 0.0
    }

    pub fn clamp_value(&self, value: f64) -> f64 {
        let mut return_val = value;
        if return_val < self.range.x() {
            return_val = self.range.x();
        }
        if return_val > self.range.y() {
            return_val = self.range.y();
        }
        return_val
    }

    pub fn localize_clamp_value(&self, value: f64) -> f64 {
        let clamped = self.clamp_value(value);
        (clamped - self.range.x()) * self.interval_div
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use crate::point_types::Point2D;
    use crate::utility::f64_equal;

    // Construct
    #[test]
    fn test_new() {
        let interval = Interval::new(Point2D::new([1.0, 2.0]));
        assert!(interval.is_valid());
        assert!(f64_equal(interval.min(), 1.0));
        assert!(f64_equal(interval.max(), 2.0));
    }
    
    #[test]
    fn test_empty() {
        let interval = Interval::empty();
        assert!(!interval.is_valid());
    }

    #[test]
    fn test_clamp() {
        let interval = Interval::new(Point2D::new([0.0, 1.0]));
        let less = interval.clamp_value(-1.0);
        let mid = interval.clamp_value(0.5);
        let more = interval.clamp_value(2.0);
        
        assert!(f64_equal(less, 0.0));
        assert!(f64_equal(mid, 0.5));
        assert!(f64_equal(more, 1.0));
    }

    #[test]
    fn test_localize_clamp() {
        let interval = Interval::new(Point2D::new([0.0, 10.0]));
        let less = interval.localize_clamp_value(-1.0);
        let mid = interval.localize_clamp_value(5.0);
        let more = interval.localize_clamp_value(11.0);
        
        assert!(f64_equal(less, 0.0), "Value was: {}", less);
        assert!(f64_equal(mid, 0.5), "Value was: {}", mid);
        assert!(f64_equal(more, 1.0), "Value was: {}", more);
    }
}