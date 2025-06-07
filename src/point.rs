use crate::mat3::Mat3;
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

impl Point3D {
    pub fn mul_mat(&self, mat: &Mat3, at: f32) -> Self {
        Point3D {
            x: self.x * mat.get_and_eval(0, 0, at).unwrap()
                + self.y * mat.get_and_eval(0, 1, at).unwrap()
                + self.z * mat.get_and_eval(0, 2, at).unwrap(),
            y: self.x * mat.get_and_eval(1, 0, at).unwrap()
                + self.y * mat.get_and_eval(1, 1, at).unwrap()
                + self.z * mat.get_and_eval(1, 2, at).unwrap(),
            z: self.x * mat.get_and_eval(2, 0, at).unwrap()
                + self.y * mat.get_and_eval(2, 1, at).unwrap()
                + self.z * mat.get_and_eval(2, 2, at).unwrap(),
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
