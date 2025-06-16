use crate::Pos4;

type Face = (usize, usize, usize);

pub trait Shape {
    fn get_points(&self) -> &[Pos4];
    fn get_triangles(&self) -> &[Face];
}

pub(crate) struct ShapeData {
    points: Vec<Pos4>,
    faces: Vec<Face>,
}

impl ShapeData {
    pub fn new(points: &[(f32, f32, f32)], faces: &[Face]) -> Self {
        let pts = points
            .iter()
            .map(|&point| Pos4::from((point.0, point.1, point.2, 1.0)))
            .collect();

        ShapeData {
            points: pts,
            faces: faces.to_vec(),
        }
    }
}

impl Shape for ShapeData {
    fn get_points(&self) -> &[Pos4] {
        &self.points
    }

    fn get_triangles(&self) -> &[Face] {
        &self.faces
    }
}
