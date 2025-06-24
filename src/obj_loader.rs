use std::{fs::File, io, io::BufReader};

use glam::{Mat4, Quat, Vec3, Vec3A, Vec4};
use obj::{Obj, Vertex, load_obj};

use crate::{Face, Pos4, model::Model};

pub fn load(
    file_name: &str,
    scale: Vec3,
    rotation: Quat,
    translation: Vec3,
) -> Result<Object, io::Error> {
    let input = BufReader::new(File::open(file_name)?);
    let model: Obj<Vertex, u32> =
        load_obj(input).map_err(io::Error::other)?;

    if model.indices.len() % 3 != 0 {
        return Err(io::Error::other(
            "indices are not a multiple of 3",
        ));
    }

    let size = model.vertices.len();
    let mut vertex = Vec::with_capacity(size);
    let mut normals = Vec::with_capacity(size);

    model.vertices.iter().for_each(|v| {
        vertex.push(Vec4::new(v.position[0], v.position[1], v.position[2], 1.0));
        normals.push(Vec3A::from_array(v.normal));
    });

    let faces = model
        .indices
        .chunks_exact(3)
        .map(|face| (face[0] as usize, face[1] as usize, face[2] as usize))
        .collect();

    Ok(Object::new(
        model,
        vertex,
        faces,
        normals,
        scale,
        rotation,
        translation,
    ))
}

pub(crate) struct Object {
    model_matrix: Mat4,
    vertex: Vec<Vec4>,
    normals: Vec<Vec3A>,
    faces: Vec<Face>,
    model: Obj<Vertex, u32>,
}

impl Object {
    pub fn new(
        model: Obj<Vertex, u32>,
        vertex: Vec<Pos4>,
        faces: Vec<Face>,
        normals: Vec<Vec3A>,
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Self {
        let model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        let mut transformed_vertex = Vec::with_capacity(vertex.len());
        for vertex in vertex.iter() {
            transformed_vertex.push(model_matrix * *vertex);
        }

        Object {
            model,
            vertex: transformed_vertex,
            faces,
            normals,
            model_matrix,
        }
    }
}

impl Model for Object {
    fn get_points(&self) -> &[Pos4] {
        &self.vertex
    }

    fn get_triangles(&self) -> &[Face] {
        &self.faces
    }
}
