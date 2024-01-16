use std::{mem::size_of, slice::from_raw_parts};
use glow::*;
use crate::{math::{vector3::Vector3, Matrix4}, renderer::GlRenderer};
use super::{Object3d, RGB};

pub trait RenderableObject {
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

    fn render(
        &mut self, 
        world_matrix: Option<&Matrix4>,
        renderer: &GlRenderer
    );
}

impl dyn RenderableObject {
    unsafe fn upload(
        &mut self, 
        renderer: &GlRenderer
    ) {
        if self.get_object().vbo.is_some() {
            return;
        }
        
        let gl = &renderer.gl;

        {
            let obj = self.get_object_mut();
            obj.vbo = Some(gl.create_buffer().unwrap());
            obj.ebo = Some(gl.create_buffer().unwrap());
            obj.vao = Some(gl.create_vertex_array().unwrap());
        }
        
        // vbos
        self.upload_vertices(gl);
        
        // ebo
        self.upload_indices(gl);

        // vao
        self.config_vao(gl, renderer.attrib_locations);
        
        // unbind
        gl.bind_vertex_array(None);
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, None);
        gl.bind_buffer(ARRAY_BUFFER, None);
    }

    unsafe fn config_vao(
        &self,
        gl: &Context,
        attrib_locations: (u32, u32)
    ) {
        let obj = self.get_object();

        gl.bind_vertex_array(obj.vao);
        
        gl.vertex_attrib_pointer_f32(
            attrib_locations.0, 
            3, 
            FLOAT, 
            false, 
            size_of::<Vector3>() as _, 
            0
        );

        gl.vertex_attrib_pointer_f32(
            attrib_locations.1, 
            3, 
            FLOAT, 
            false, 
            size_of::<RGB>() as _, 
            (size_of::<Vector3>() * obj.positions.len()) as _
        );
    }

    unsafe fn upload_indices(
        &self, 
        gl: &Context 
    ) {
        let obj = self.get_object();

        let indices = from_raw_parts(
            obj.indices.as_ptr() as *const u8,
            size_of::<u32>() * obj.indices.len()
        );

        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, obj.ebo);
        gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, indices, STATIC_DRAW);
    }

    unsafe fn upload_vertices(
        &self, 
        gl: &Context 
    ) {
        let obj = self.get_object();

        let num_vertices = obj.positions.len();
        
        let positions = from_raw_parts(
            obj.positions.as_ptr() as *const u8,
            size_of::<Vector3>() * num_vertices
        );

        let colors = from_raw_parts(
            obj.colors.as_ptr() as *const u8,
            size_of::<RGB>() * num_vertices
        );

        gl.bind_buffer(ARRAY_BUFFER, obj.vbo);
        gl.buffer_data_size(ARRAY_BUFFER, (positions.len() + colors.len()) as _, STATIC_DRAW);
        gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, 0, positions);
        gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, positions.len() as _, colors);
    }

    unsafe fn bind(
        &self,
        renderer: &GlRenderer
    ) {
        let obj = self.get_object();
        let gl = &renderer.gl;

        gl.bind_vertex_array(obj.vao);
        gl.enable_vertex_attrib_array(renderer.attrib_locations.0);
        gl.enable_vertex_attrib_array(renderer.attrib_locations.1);

        gl.bind_buffer(ARRAY_BUFFER, obj.vbo);
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, obj.ebo);
    }

    unsafe fn unbind(
        &self,
        renderer: &GlRenderer
    ) {
        let gl = &renderer.gl;
        
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, None);
        gl.bind_buffer(ARRAY_BUFFER, None);

        gl.disable_vertex_attrib_array(renderer.attrib_locations.1);
        gl.disable_vertex_attrib_array(renderer.attrib_locations.0);
        gl.bind_vertex_array(None);
    }

    unsafe fn update(
        &mut self,
        world_matrix: Option<&Matrix4>,
        renderer: &GlRenderer
    ) -> bool {
        self.upload(renderer);

        let obj = self.get_object_mut();

        let mut updated = obj.dirt;
        obj.update_matrix();

        if let Some(world_matrix) = world_matrix {
            updated = true;
            obj.world_matrix = world_matrix.mul(&obj.matrix);
        } 
        else {
            if updated {
                obj.world_matrix = obj.matrix.clone();
            }
        }

        let gl = &renderer.gl;

        gl.uniform_matrix_4_f32_slice(
            Some(&renderer.uniform_locations.model_view), 
            false, 
            obj.world_matrix.to_slice()
        );

        updated
    }

    pub fn draw(
        &mut self,
        world_matrix: Option<&Matrix4>,
        renderer: &GlRenderer
    ) {
        unsafe {
            let updated = self.update(world_matrix, renderer);
            self.bind(renderer);
            
            let obj = self.get_object();
            let gl = &renderer.gl;

            gl.draw_elements(
                obj.mode as _, 
                obj.indices.len() as _, 
                UNSIGNED_INT, 
                0
            );

            self.unbind(renderer);

            let world_matrix = if updated {
                Some(&obj.world_matrix)
            } 
            else {
                None
            };

            for child in &obj.children {
                child.borrow_mut().render(
                    world_matrix,
                    renderer
                );
            }
        }
    }

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

