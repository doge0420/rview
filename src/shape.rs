use crate::Point3D;

pub trait Shape {
    fn new(points: &[(f32, f32, f32)], faces: &[(usize, usize, usize)]) -> Self;
    fn get_points(&self) -> &[Point3D];
}

pub(crate) struct ShapeData {
    points: Vec<Point3D>,
    faces: Vec<(usize, usize, usize)>,
}

impl Shape for ShapeData {
    fn new(points: &[(f32, f32, f32)], faces: &[(usize, usize, usize)]) -> Self {
        let pts = points.iter().map(|&point| Point3D::from(point)).collect();

        ShapeData {
            points: pts,
            faces: faces.to_vec(),
        }
    }

    fn get_points(&self) -> &[Point3D] {
        &self.points
    }
}
