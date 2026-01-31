use nurbs::vector_3::Vec3;

pub struct Ray3 {
    origin: Vec3,
    dir: Vec3,
    time: f64,
}

impl Ray3 {
    pub fn new(origin: Vec3, dir: Vec3, time: f64) -> Ray3 {
        Ray3 { origin, dir, time }
    }

    pub fn empty() -> Ray3 {
        Ray3::new(Vec3::empty(), Vec3::empty(), 0.0)
    }

    pub fn copy(&self) -> Ray3 {
        Ray3::new(self.origin.clone(), self.dir.clone(), self.time)
    }

    pub fn origin(&self) -> Vec3 {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, time: f64) -> Vec3 {
        self.origin() + (time * self.direction())
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        self.dir = direction;
    }

    pub fn set_time(&mut self, time: f64) {
        self.time = time;
    }
}
