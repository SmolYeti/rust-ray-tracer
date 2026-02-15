#[derive(Debug, Clone, Copy)]
pub struct Point<const N: usize> {
    values: [f64; N],
}

pub type Point2D = Point<2>;
pub type Point3D = Point<3>;
pub type Point4D = Point<4>;

pub mod point;
pub mod point_2d;
pub mod point_3d;
pub mod point_4d;
