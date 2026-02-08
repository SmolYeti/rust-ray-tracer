use crate::point_types::Point2D;

#[derive(Debug, Clone, Copy)]
pub struct Interval<const N: usize> {
    pub range: Point2D,
    pub interval_div: f64,
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

    pub fn is_valid(&self) -> bool {
        self.interval_div > 0.0
    }

    pub fn clamp_value(&self, value: f64) -> f64 {
        let mut return_val = self.value;
        if (return_val < self.range.x()) {
            return_val = self.range.x();
        }
        if (return_val > self.range.y()) {
            return_val = self.range.y()
        }
        return_val
    }

    pub fn localize_clamp_value(&self, value: f64) -> f64 {
        let clamped = self.clamp_value(value);
        (clamped - self.range.x()) * self.interval_div
    }
}