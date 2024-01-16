use glow::{NativeBuffer, NativeVertexArray};
use crate::math::Vector3;
use super::RGB;

pub struct BufferGeometry {
    pub(crate) mode: u32,
    pub(crate) indices: Vec<u32>,
    pub(crate) positions: Vec<Vector3>,
    pub(crate) colors: Vec<RGB>,
    
    pub(crate) vbo: Option<NativeBuffer>,
    pub(crate) ebo: Option<NativeBuffer>,
    pub(crate) vao: Option<NativeVertexArray>,
}

impl BufferGeometry {
    pub fn new(
        mode: u32,
        indices: Vec<u32>,
        positions: Vec<Vector3>,
        colors: Vec<RGB>,
) -> Self {
        Self { 
            mode,
            indices,
            positions,
            colors,
            vbo: None,
            ebo: None,
            vao: None,
        }
    }
}

impl Clone for BufferGeometry {
    fn clone(
        &self
    ) -> Self {
        Self { 
            mode: self.mode, 
            indices: self.indices.clone(), 
            positions: self.positions.clone(), 
            colors: self.colors.clone(), 
            vbo: None, 
            ebo: None, 
            vao: None, 
        }
    }
}

