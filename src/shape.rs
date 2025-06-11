use crate::Pos3;

type Face = (usize, usize, usize);

pub trait Shape {
    fn new(points: &[(f32, f32, f32)], faces: &[Face]) -> Self;
    fn get_points(&self) -> &[Pos3];
    fn get_triangles(&self) -> &[Face];
}

pub(crate) struct ShapeData {
    points: Vec<Pos3>,
    faces: Vec<Face>,
}

impl Shape for ShapeData {
    fn new(points: &[(f32, f32, f32)], faces: &[Face]) -> Self {
        assert!(
            faces.len() % 3 == 0,
            "The shape's faces count isn't a multiple of 3 (needed because of triangles)"
        );

        let pts = points.iter().map(|&point| Pos3::from(point)).collect();

        ShapeData {
            points: pts,
            faces: faces.to_vec(),
        }
    }

    fn get_points(&self) -> &[Pos3] {
        &self.points
    }

    fn get_triangles(&self) -> &[Face] {
        &self.faces
    }
}
