use std::ops::Mul;

#[derive(Debug)]
pub(crate) struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Mul<f32> for Point2D {
    type Output = Point2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Point2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(f32, f32)> for Point2D {
    fn from(tuple: (f32, f32)) -> Self {
        Point2D {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Mul<f32> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f32) -> Self::Output {
        Point3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl From<(f32, f32, f32)> for Point3D {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Point3D {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}
