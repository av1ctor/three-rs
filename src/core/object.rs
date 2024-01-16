use glow::*;
use crate::renderer::GlRenderer;
use super::Object3d;

pub trait Object {
    fn get_object(
        &self
    ) -> &Object3d;

    fn get_object_mut(
        &mut self
    ) -> &mut Object3d;

    fn drop(
        &mut self, 
        renderer: &GlRenderer
    );
}

impl dyn Object {
    pub fn delete(
        &mut self, 
        renderer: &GlRenderer
    ) {
        unsafe {
            let obj = self.get_object();
            let gl = &renderer.gl;

            if let Some(vao) = obj.vao {
                gl.delete_vertex_array(vao);
            }
            if let Some(ebo) = obj.ebo {
                gl.delete_buffer(ebo);
            }
            if let Some(vbo) = obj.vbo {
                gl.delete_buffer(vbo);
            }
        }
    }
}

