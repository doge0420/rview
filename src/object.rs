use std::rc::Rc;

use glam::{Mat4, Quat, Vec3, Vec4};

use crate::shape::Shape;

pub(crate) struct Object<O>
where
    O: Shape,
{
    shape: Rc<O>,
    model_matrix: Mat4,
    tranformed_vertices: Vec<Vec4>,
    dirty: bool,
}

impl<O> Object<O>
where
    O: Shape,
{
    pub fn new(shape: Rc<O>, scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        Object {
            shape,
            model_matrix,
            tranformed_vertices: Vec::new(),
            dirty: true,
        }
    }

    pub fn evolve(&mut self, scale: Vec3, rotation: Quat, translation: Vec3) {
        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        self.dirty = true;
    }

    pub fn get_vertices(&mut self) -> &[Vec4] {
        if self.dirty {
            let points = self.shape.get_points();

            if self.tranformed_vertices.len() != points.len() {
                self.tranformed_vertices.resize(points.len(), Vec4::ZERO);
            }

            for (i, vertex) in points.iter().enumerate() {
                self.tranformed_vertices[i] = self.model_matrix * vertex.clone();
            }

            self.dirty = false;
        }

        &self.tranformed_vertices
    }

    #[inline(always)]
    pub fn get_triangles(&self) -> &[(usize, usize, usize)] {
        self.shape.get_triangles()
    }
}
