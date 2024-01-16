use glow::*;
use crate::renderer::GlRenderer;
use super::BufferGeometry;

pub trait Geometrical {
    fn get_geometry(
        &self
    ) -> &BufferGeometry;

    fn get_geometry_mut(
        &mut self
    ) -> &mut BufferGeometry;

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
        unsafe {
            let geo = self.get_geometry();
            let gl = &renderer.gl;

            if let Some(vao) = geo.vao {
                gl.delete_vertex_array(vao);
            }
            if let Some(ebo) = geo.ebo {
                gl.delete_buffer(ebo);
            }
            if let Some(vbo) = geo.vbo {
                gl.delete_buffer(vbo);
            }
        }
    }
}

