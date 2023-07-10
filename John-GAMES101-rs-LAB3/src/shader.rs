use std::rc::Rc;
use nalgebra::{Vector2, Vector3};
use crate::texture::Texture;

pub struct FragmentShaderPayload<'a> {
    pub view_pos: Vector3<f64>,
    pub color: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub tex_coords: Vector2<f64>,
    pub texture: Option<Rc<&'a Texture>>,
}

impl<'a> FragmentShaderPayload<'a> {
    pub fn new(col: &Vector3<f64>, nor: &Vector3<f64>, tc: &Vector2<f64>, tex: Option<Rc<&'a Texture>>) -> Self {
        FragmentShaderPayload {
            view_pos: Vector3::zeros(),
            color: col.clone(),
            normal: nor.clone(),
            tex_coords: tc.clone(),
            texture: match tex {
                None => None,
                Some(rc) => Some(rc.clone()),
            },
        }
    }
}

pub struct VertexShaderPayload {
    pub position: Vector3<f64>,
}