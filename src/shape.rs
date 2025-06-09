use crate::Pos3;

pub trait Shape {
    fn new(points: &[(f32, f32, f32)], faces: &[(usize, usize, usize)]) -> Self;
    fn get_points(&self) -> &[Pos3];
}

pub(crate) struct ShapeData {
    points: Vec<Pos3>,
    faces: Vec<(usize, usize, usize)>,
}

impl Shape for ShapeData {
    fn new(points: &[(f32, f32, f32)], faces: &[(usize, usize, usize)]) -> Self {
        let pts = points.iter().map(|&point| Pos3::from(point)).collect();

        ShapeData {
            points: pts,
            faces: faces.to_vec(),
        }
    }

    fn get_points(&self) -> &[Pos3] {
        &self.points
    }
}
