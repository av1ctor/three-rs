use std::mem::size_of;

use glow::{NativeBuffer, NativeVertexArray};
use crate::math::Vector3;
use super::RGB;

pub struct BufferGeometrySizes {
    pub positions: usize,
    pub colors: usize,
    pub total: usize,
}

pub struct BufferGeometry {
    pub(crate) mode: u32,
    pub(crate) indices: Option<Vec<u32>>,
    pub(crate) positions: Option<Vec<Vector3>>,
    pub(crate) colors: Option<Vec<RGB>>,
    
    pub(crate) vbo: Option<NativeBuffer>,
    pub(crate) ebo: Option<NativeBuffer>,
    pub(crate) vao: Option<NativeVertexArray>,
}

impl BufferGeometry {
    pub fn new(
        mode: u32,
        indices: Option<Vec<u32>>,
        positions: Option<Vec<Vector3>>,
        colors: Option<Vec<RGB>>,
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

    pub fn get_sizes(
        &self
    ) -> BufferGeometrySizes {
        
        let mut sizes = BufferGeometrySizes {
            positions: 0,
            colors: 0,
            total: 0,
        };

        if let Some(positions) = &self.positions {
            sizes.positions = positions.len() * size_of::<Vector3>();
            sizes.total += sizes.positions;
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
            colors: self.colors.clone(), 
            vbo: None, 
            ebo: None, 
            vao: None, 
        }
    }
}

