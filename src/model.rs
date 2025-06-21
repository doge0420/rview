use glam::Vec3A;

use crate::{Face, Pos4};

pub(crate) trait Model {
    fn get_points(&self) -> &[Pos4];
    fn get_triangles(&self) -> &[Face];
    fn get_normals(&self) -> &[Vec3A];
}
