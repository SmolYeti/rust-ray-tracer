use crate::vector_3::Vec3;

pub struct OrthonormalBasis {
    axis: [Vec3; 3],
}

impl OrthonormalBasis {
    pub fn new(normal: &Vec3) -> OrthonormalBasis {
        let mut axis: [Vec3; 3] = [Vec3::empty(); 3];
        axis[2] = normal.unit_vector();
        let a: Vec3 = if axis[2].x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        axis[1] = axis[2].cross(&a).unit_vector();
        axis[0] = axis[2].cross(&axis[1]);

        OrthonormalBasis { axis }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn transform(&self, point: Vec3) -> Vec3 {
        point.x * self.axis[0] + point.y * self.axis[1] + point.z * self.axis[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_normal() {
        let normal = Vec3::new(1.0, 0.0, 0.0);
        let result = OrthonormalBasis::new(&normal);
        assert!((result.u().y + 1.0).abs() < f64::EPSILON);
        assert!((result.v().z - 1.0).abs() < f64::EPSILON);
        assert!((result.w().x - 1.0).abs() < f64::EPSILON);
    }
}
