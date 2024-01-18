use std::mem::size_of;

use glow::{NativeBuffer, NativeVertexArray};
use crate::math::Vector3;
use super::RGB;

pub(crate) struct BufferAttributeSizes {
    pub positions: usize,
    pub normals: usize,
    pub colors: usize,
    pub total: usize,
}

pub struct BufferGeometry {
    pub(crate) mode: u32,
    pub(crate) indices: Option<Vec<u32>>,
    pub(crate) positions: Option<Vec<Vector3>>,
    pub(crate) normals: Option<Vec<Vector3>>,
    pub(crate) colors: Option<Vec<RGB>>,
    pub(crate) dirt: bool,
    
    pub(crate) vbo: Option<NativeBuffer>,
    pub(crate) ebo: Option<NativeBuffer>,
    pub(crate) vao: Option<NativeVertexArray>,
}

impl BufferGeometry {
    pub fn new(
        mode: u32,
        indices: Option<Vec<u32>>,
        positions: Option<Vec<Vector3>>,
        normals: Option<Vec<Vector3>>,
        colors: Option<Vec<RGB>>,
) -> Self {
        Self { 
            mode,
            indices,
            positions,
            normals,
            colors,
            dirt: false,
            vbo: None,
            ebo: None,
            vao: None,
        }
    }

    pub(crate) fn get_attribute_sizes(
        &self
    ) -> BufferAttributeSizes {
        
        let mut sizes = BufferAttributeSizes {
            positions: 0,
            normals: 0,
            colors: 0,
            total: 0,
        };

        if let Some(positions) = &self.positions {
            sizes.positions = positions.len() * size_of::<Vector3>();
            sizes.total += sizes.positions;
        }

        if let Some(normals) = &self.normals {
            sizes.normals = normals.len() * size_of::<Vector3>();
            sizes.total += sizes.normals;
        }

        if let Some(colors) = &self.colors {
            sizes.colors = colors.len() * size_of::<RGB>();
            sizes.total += sizes.colors;
        }

        sizes
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
            normals: self.normals.clone(), 
            colors: self.colors.clone(), 
            dirt: false,
            vbo: None, 
            ebo: None, 
            vao: None, 
        }
    }
}

