use std::{mem::size_of, slice::from_raw_parts};
use glow::*;
use crate::{
    math::{vector3::Vector3, Matrix4}, 
    renderer::GlRenderer, 
    camera::ObjectCamera
};
use super::{RGB, Object3d, Geometrical};

pub trait Renderable
    where Self: Object3d + Geometrical {
    fn render(
        &mut self, 
        world_matrix: Option<&Matrix4>,
        camera: &dyn ObjectCamera,
        renderer: &GlRenderer
    );
}

impl dyn Renderable {
    unsafe fn upload(
        &mut self, 
        renderer: &GlRenderer
    ) {
        if self.get_geometry().vbo.is_some() {
            return;
        }
        
        let gl = &renderer.gl;

        {
            let geo = self.get_geometry_mut();
            geo.vbo = Some(gl.create_buffer().unwrap());
            geo.ebo = Some(gl.create_buffer().unwrap());
            geo.vao = Some(gl.create_vertex_array().unwrap());
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
        let geo = self.get_geometry();

        let sizes = geo.get_sizes();
        let mut offset = 0;
        
        if sizes.total > 0 {
            gl.bind_vertex_array(geo.vao);

            if sizes.positions > 0 {
                gl.vertex_attrib_pointer_f32(
                    attrib_locations.0, 
                    3, 
                    FLOAT, 
                    false, 
                    size_of::<Vector3>() as _, 
                    offset as _
                );
                offset += sizes.positions;
            }

            if sizes.colors > 0 {
                gl.vertex_attrib_pointer_f32(
                    attrib_locations.1, 
                    3, 
                    FLOAT, 
                    false, 
                    size_of::<RGB>() as _, 
                    offset as _
                );
            }
        }
    }

    unsafe fn upload_indices(
        &self, 
        gl: &Context 
    ) {
        let geo = self.get_geometry();

        if let Some(indices) = &geo.indices {
            let buffer = from_raw_parts(
                indices.as_ptr() as *const u8,
                size_of::<u32>() * indices.len()
            );
            
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, geo.ebo);
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, buffer, STATIC_DRAW);
        }
    }

    unsafe fn upload_vertices(
        &self, 
        gl: &Context 
    ) {
        let geo = self.get_geometry();

        let sizes = geo.get_sizes();
        let mut offset = 0;
        
        if sizes.total > 0 {
            gl.bind_buffer(ARRAY_BUFFER, geo.vbo);
            gl.buffer_data_size(ARRAY_BUFFER, sizes.total as _, STATIC_DRAW);
            
            if let Some(positions) = &geo.positions { 
                let buffer = from_raw_parts(
                    positions.as_ptr() as *const u8,
                    sizes.positions
                );
                gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, offset as _, buffer);
                offset += sizes.positions;
            }

            if let Some(colors) = &geo.colors { 
                let buffer = from_raw_parts(
                    colors.as_ptr() as *const u8,
                    sizes.colors
                );

                gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, offset as _, buffer);
                //offset += sizes.colors;
            }
        }
    }

    unsafe fn bind(
        &self,
        renderer: &GlRenderer
    ) {
        let geo = self.get_geometry();
        let gl = &renderer.gl;

        gl.bind_vertex_array(geo.vao);
        gl.enable_vertex_attrib_array(renderer.attrib_locations.0);
        gl.enable_vertex_attrib_array(renderer.attrib_locations.1);

        gl.bind_buffer(ARRAY_BUFFER, geo.vbo);
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, geo.ebo);
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
        camera: &dyn ObjectCamera,
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

        let model_view_matrix = camera.get_data()
            .world_matrix_inverse.mul(&obj.world_matrix);

        let gl = &renderer.gl;

        gl.uniform_matrix_4_f32_slice(
            Some(&renderer.uniform_locations.model_view), 
            false, 
            model_view_matrix.to_slice()
        );

        updated
    }

    pub fn draw(
        &mut self,
        world_matrix: Option<&Matrix4>,
        camera: &dyn ObjectCamera,
        renderer: &GlRenderer
    ) {
        unsafe {
            let updated = self.update(
                world_matrix, camera, 
                renderer
            );
            self.bind(renderer);
            
            let obj = self.get_object();
            let geo = self.get_geometry();
            let gl = &renderer.gl;

            if let Some(indices) = &geo.indices {
                gl.draw_elements(
                    geo.mode as _, 
                    indices.len() as _, 
                    UNSIGNED_INT, 
                    0
                );
            }
            else if let Some(positions) = &geo.positions {
                gl.draw_arrays(
                    geo.mode as _, 
                    0, 
                    positions.len() as _
                );
            }

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
                    camera,
                    renderer
                );
            }
        }
    }
}

