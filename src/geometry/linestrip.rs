use crate::{
    core::{BufferGeometry, Geometrical, RGB, BufferGeometryMode}, 
    math::Vector3
};

#[derive(Clone)]
pub struct LineStrip {
    pub geo: BufferGeometry,
}

impl LineStrip {
    pub fn new(
        lines: Vec<Vector3>,
        _color: RGB,
    ) -> Self {
        
        Self {
            geo: BufferGeometry::new(
                BufferGeometryMode::LineStrip, 
                None, 
                Some(lines), 
                None,
                None,
            )
        }
    }
}

impl Geometrical for LineStrip {
    fn get_geometry(
        &self
    ) -> &BufferGeometry {
        &self.geo
    }

    fn get_geometry_mut(
        &mut self
    ) -> &mut BufferGeometry {
        &mut self.geo
    }

    fn drop(
        &mut self, 
        renderer: &crate::renderer::GlRenderer
    ) {
        (self as &mut dyn Geometrical).destroy(renderer)
    }
}
