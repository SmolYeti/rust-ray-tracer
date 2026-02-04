use core::ops;
use crate::point_types::Point;

impl<const N: usize> Point<N> {
    pub fn new(values: [f64; N]) -> Point<N> {
        Point { values }
    }

    pub fn empty() -> Point<N> {
        Point::new([0.0; N])
    }
}

// Negate
impl<const N: usize> ops::Neg for Point<N> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut values = self.values;
        for n in 0..N {
            values[n] = -values[n];
        }

        Self { values }
    }
}

impl<'a, const N: usize> ops::Neg for &'a Point<N> {
    type Output = Point<N>;

    fn neg(self) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] = -values[n];
        }

        Point { values }
    }
}

// Add
impl<const N: usize> ops::Add<Self> for Point<N> {
    type Output = Point<N>;

    fn add(self, _rhs: Self) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] += _rhs.values[n];
        }

        Point { values }
    }
}

impl<'a, 'b, const N: usize> ops::Add<&'a Self> for &'b Point<N> {
    type Output = Point<N>;

    fn add(self, _rhs: &Self) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] += _rhs.values[n];
        }

        Point { values }
    }
}

impl<const N: usize> ops::Add<f64> for Point<N> {
    type Output = Point<N>;

    fn add(self, _rhs: f64) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] += _rhs;
        }

        Point { values }
    }
}

impl<'a, const N: usize> ops::Add<f64> for &'a Point<N> {
    type Output = Point<N>;

    fn add(self, _rhs: f64) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] += _rhs;
        }

        Point { values }
    }
}

impl<const N: usize> ops::Add<Point<N>> for f64 {
    type Output = Point<N>;

    fn add(self, _rhs: Point<N>) -> Point<N> {
        let mut values = _rhs.values;
        for n in 0..N {
            values[n] += self;
        }

        Point { values }
    }
}

impl<'a, const N: usize> ops::Add<&'a Point<N>> for f64 {
    type Output = Point<N>;

    fn add(self, _rhs: &'a Point<N>) -> Point<N> {
        let mut values = _rhs.values;
        for n in 0..N {
            values[n] += self;
        }

        Point { values }
    }
}


// Add Assign
impl<const N: usize> ops::AddAssign<Self> for Point<N> {
    fn add_assign(&mut self, rhs: Self) {
        for n in 0..N {
            self.values[n] += rhs.values[n];
        }
    }
}

impl<const N: usize> ops::AddAssign<f64> for Point<N> {
    fn add_assign(&mut self, rhs: f64) {
        for n in 0..N {
            self.values[n] += rhs;
        }
    }
}

// Subtract
impl<const N: usize> ops::Sub<Point<N>> for Point<N> {
    type Output = Point<N>;

    fn sub(self, _rhs: Point<N>) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] -= _rhs.values[n];
        }

        Point { values }
    }
}

impl<'a, 'b, const N: usize> ops::Sub<&'a Point<N>> for &'b Point<N> {
    type Output = Point<N>;

    fn sub(self, _rhs: &'a Point<N>) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] -= _rhs.values[n];
        }

        Point { values }
    }
}

impl<const N: usize> ops::Sub<f64> for Point<N> {
    type Output = Point<N>;

    fn sub(self, _rhs: f64) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] -= _rhs;
        }

        Point { values }
    }
}

impl<'a, const N: usize> ops::Sub<f64> for &'a Point<N> {
    type Output = Point<N>;

    fn sub(self, _rhs: f64) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] -= _rhs;
        }

        Point { values }
    }
}

impl<const N: usize> ops::Sub<Point<N>> for f64 {
    type Output = Point<N>;

    fn sub(self, _rhs: Point<N>) -> Point<N> {
        let mut values = [self; N];
        for n in 0..N {
            values[n] -= _rhs.values[n];
        }

        Point { values }
    }
}

impl<'a, const N: usize> ops::Sub<&'a Point<N>> for f64 {
    type Output = Point<N>;

    fn sub(self, _rhs: &'a Point<N>) -> Point<N> {
        let mut values = [self; N];
        for n in 0..N {
            values[n] -= _rhs.values[n];
        }

        Point { values }
    }
}

// Subtract Assign
impl<const N: usize> ops::SubAssign<Self> for Point<N> {
    fn sub_assign(&mut self, rhs: Self) {
        for n in 0..N {
            self.values[n] -= rhs.values[n];
        }
    }
}

impl<const N: usize> ops::SubAssign<f64> for Point<N> {
    fn sub_assign(&mut self, rhs: f64) {
        for n in 0..N {
            self.values[n] -= rhs;
        }
    }
}

// Multiply
impl<const N: usize> ops::Mul<Self> for Point<N> {
    type Output = Point<N>;

    fn mul(self, _rhs: Self) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] *= _rhs.values[n];
        }

        Point { values }
    }
}

impl<'a, 'b, const N: usize> ops::Mul<&'a Self> for &'b Point<N> {
    type Output = Point<N>;

    fn mul(self, _rhs: &Self) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] *= _rhs.values[n];
        }

        Point { values }
    }
}

impl<const N: usize> ops::Mul<f64> for Point<N> {
    type Output = Point<N>;

    fn mul(self, _rhs: f64) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] *= _rhs;
        }

        Point { values }
    }
}

impl<'a, const N: usize> ops::Mul<f64> for &'a Point<N> {
    type Output = Point<N>;

    fn mul(self, _rhs: f64) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] *= _rhs;
        }

        Point { values }
    }
}

impl<const N: usize> ops::Mul<Point<N>> for f64 {
    type Output = Point<N>;

    fn mul(self, _rhs: Point<N>) -> Point<N> {
        let mut values = _rhs.values;
        for n in 0..N {
            values[n] *= self;
        }

        Point { values }
    }
}

impl<'a, const N: usize> ops::Mul<&'a Point<N>> for f64 {
    type Output = Point<N>;

    fn mul(self, _rhs: &'a Point<N>) -> Point<N> {
        let mut values = _rhs.values;
        for n in 0..N {
            values[n] *= self;
        }

        Point { values }
    }
}

// Divide
impl<const N: usize> ops::Div<Point<N>> for Point<N> {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self {
        let mut values = self.values;
        for n in 0..N {
            values[n] /= _rhs.values[n];
        }

        Point { values }
    }
}

impl<'a, 'b, const N: usize> ops::Div<&'a Point<N>> for &'b Point<N> {
    type Output = Point<N>;

    fn div(self, _rhs: &'a Point<N>) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] /= _rhs.values[n];
        }

        Point { values }
    }
}

impl<const N: usize> ops::Div<f64> for Point<N> {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self {
        let mut values = self.values;
        for n in 0..N {
            values[n] /= _rhs;
        }

        Point { values }
    }
}

impl<'a, const N: usize> ops::Div<f64> for &'a Point<N> {
    type Output = Point<N>;

    fn div(self, _rhs: f64) -> Point<N> {
        let mut values = self.values;
        for n in 0..N {
            values[n] /= _rhs;
        }

        Point { values }
    }
}

impl<const N: usize> ops::Div<Point<N>> for f64 {
    type Output = Point<N>;

    fn div(self, _rhs: Point<N>) -> Point<N> {
        let mut values = [self; N];
        for n in 0..N {
            values[n] /= _rhs.values[n];
        }

        Point { values }
    }
}

impl<'a, const N: usize> ops::Div<&'a Point<N>> for f64 {
    type Output = Point<N>;

    fn div(self, _rhs: &'a Point<N>) -> Point<N> {
        let mut values = [self; N];
        for n in 0..N {
            values[n] /= _rhs.values[n];
        }

        Point { values }
    }
}