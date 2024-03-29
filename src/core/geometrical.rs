use crate::{renderer::GlRenderer, math::Vector3};
use super::BufferGeometry;

pub trait Geometrical {
    fn get_geometry(
        &self
    ) -> &BufferGeometry;

    fn get_geometry_mut(
        &mut self
    ) -> &mut BufferGeometry;

    fn set_positions(
        &mut self,
        positions: Vec<Vector3>
    ) {
        let geo = &mut self.get_geometry_mut();
        geo.positions = Some(positions);
        geo.dirt = true;
    }

    fn drop(
        &mut self, 
        renderer: &GlRenderer
    );
}

impl dyn Geometrical {
    pub fn destroy(
        &mut self, 
        renderer: &GlRenderer
    ) {
        renderer.delete_buffers(self.get_geometry());
    }
}

