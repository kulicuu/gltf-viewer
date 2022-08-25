use std::path::Path;

use collision::{Aabb, Aabb3, Union};

use gltf;


use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};


use std::sync::Arc;

use crate::render::math::*;
use crate::render::root::Root;
// use crate::import_data::ImportData;
use crate::render::texture::ImportData;
use crate::render::primitive::Primitive;

pub struct Mesh {
    pub index: usize, // glTF index
    pub primitives: Vec<Primitive>,
    // TODO: weights
    // pub weights: Vec<Rc<?>>
    pub name: Option<String>,
    pub bounds: Aabb3<f32>,
}

impl Mesh {
    pub fn from_gltf(
        gl: Arc<GL>,
        g_mesh: &gltf::Mesh,
        root: &mut Root,
        imp: &ImportData,
        base_path: &Path,
    ) -> Mesh {
        let primitives: Vec<Primitive> = g_mesh.primitives()
            .enumerate()
            .map(|(i, g_prim)| {
                Primitive::from_gltf(
                    gl.clone(),
                    &g_prim, i, g_mesh.index(), root, imp, base_path)
            })
            .collect();

        let bounds = primitives.iter()
            .fold(Aabb3::zero(), |bounds, prim| prim.bounds.union(&bounds));

        Mesh {
            index: g_mesh.index(),
            primitives: primitives,
            name: g_mesh.name().map(|s| s.into()),
            bounds,
        }
    }

    pub fn draw(&self, model_matrix: &Matrix4, mvp_matrix: &Matrix4, camera_position: &Vector3) {
        for primitive in &self.primitives {
            unsafe { primitive.draw(model_matrix, mvp_matrix, camera_position) }
        }
    }
}
