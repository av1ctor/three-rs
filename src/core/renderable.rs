use std::{mem::size_of, slice::from_raw_parts, collections::HashMap};
use glow::*;
use crate::{
    math::{vector3::Vector3, Matrix4}, 
    renderer::{GlRenderer, ShaderProgram, ShaderProgramType, ShaderUniformType}, 
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
        self.config_vao(gl);
        
        // unbind
        gl.bind_vertex_array(None);
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, None);
        gl.bind_buffer(ARRAY_BUFFER, None);
    }

    unsafe fn config_vao(
        &self,
        gl: &Context
    ) {
        let geo = self.get_geometry();

        let sizes = geo.get_attribute_sizes();
        let mut offset = 0;
        let mut location = 0;
        
        if sizes.total > 0 {
            gl.bind_vertex_array(geo.vao);

            if sizes.positions > 0 {
                gl.vertex_attrib_pointer_f32(
                    location, 
                    3, 
                    FLOAT, 
                    false, 
                    size_of::<Vector3>() as _, 
                    offset as _
                );
                offset += sizes.positions;
                location += 1;
            }

            if sizes.normals > 0 {
                gl.vertex_attrib_pointer_f32(
                    location, 
                    3, 
                    FLOAT, 
                    false, 
                    size_of::<Vector3>() as _, 
                    offset as _
                );
                offset += sizes.normals;
                location += 1;
            }

            if sizes.colors > 0 {
                gl.vertex_attrib_pointer_f32(
                    location, 
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
        &mut self, 
        gl: &Context
    ) {
        let geo = self.get_geometry_mut();

        let sizes = geo.get_attribute_sizes();
        
        if sizes.total > 0 {
            gl.bind_buffer(ARRAY_BUFFER, geo.vbo);
            gl.buffer_data_size(ARRAY_BUFFER, sizes.total as _, STATIC_DRAW);
            
            let mut offset = 0;
            if let Some(positions) = &geo.positions { 
                let buffer = from_raw_parts(
                    positions.as_ptr() as *const u8,
                    sizes.positions
                );
                gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, offset as _, buffer);
                offset += sizes.positions;
            }

            if let Some(normals) = &geo.normals { 
                let buffer = from_raw_parts(
                    normals.as_ptr() as *const u8,
                    sizes.normals
                );
                gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, offset as _, buffer);
                offset += sizes.normals;
            }

            if let Some(colors) = &geo.colors { 
                let buffer = from_raw_parts(
                    colors.as_ptr() as *const u8,
                    sizes.colors
                );

                gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, offset as _, buffer);
                //offset += sizes.colors;
            }

            geo.dirt = false;
        }
    }

    unsafe fn bind(
        &self,
        renderer: &GlRenderer
    ) {
        let geo = self.get_geometry();
        let gl = &renderer.gl;

        gl.bind_vertex_array(geo.vao);

        let mut location = 0;
        if geo.positions.is_some() { 
            gl.enable_vertex_attrib_array(location);
            location += 1;
        }
        if geo.normals.is_some() { 
            gl.enable_vertex_attrib_array(location);
            location += 1;
        }
        if geo.colors.is_some() { 
            gl.enable_vertex_attrib_array(location);
            //location += 1;
        }

        gl.bind_buffer(ARRAY_BUFFER, geo.vbo);
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, geo.ebo);
    }

    unsafe fn unbind(
        &self,
        renderer: &GlRenderer
    ) {
        let geo = self.get_geometry();
        let gl = &renderer.gl;
        
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, None);
        gl.bind_buffer(ARRAY_BUFFER, None);

        let mut location = 0;
        if geo.positions.is_some() { 
            gl.disable_vertex_attrib_array(location);
            location += 1;
        }
        if geo.normals.is_some() { 
            gl.disable_vertex_attrib_array(location);
            location += 1;
        }
        if geo.colors.is_some() { 
            gl.disable_vertex_attrib_array(location);
            //location += 1;
        }

        gl.bind_vertex_array(None);

        gl.use_program(None);
    }

    unsafe fn update(
        &mut self,
        world_matrix: Option<&Matrix4>,
        camera: &dyn ObjectCamera,
        program: &ShaderProgram,
        renderer: &GlRenderer
    ) -> bool {
        self.upload(renderer);

        let gl = &renderer.gl;
        let geo = self.get_geometry();
        
        if geo.dirt {
            self.upload_vertices(gl);
        }

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

        // update matrices
        let model_view_matrix = camera.get_data()
            .world_matrix_inverse.mul(&obj.world_matrix);

        gl.uniform_matrix_4_f32_slice(
            Some(&program.uniform_locations.projection), 
            false, 
            camera.get_data().proj_matrix.to_slice()
        );

        gl.uniform_matrix_4_f32_slice(
            Some(&program.uniform_locations.model_view), 
            false, 
            model_view_matrix.to_slice()
        );

        // update uniforms depending on the shader used
        let mut uniform_values = HashMap::<String, Vec<f32>>::default();
        uniform_values.insert("color".to_string(), Vector3::new(1.0, 0.0, 0.0).to_slice().to_vec());

        for (name, uniform) in &program.uniform_locations.other {
            match uniform.ty {
                ShaderUniformType::Vector3 => {
                    gl.uniform_3_f32_slice(
                        Some(&uniform.location), &uniform_values[name]
                    );
                },
                ShaderUniformType::Matrix4 => {
                    gl.uniform_matrix_4_f32_slice(
                        Some(&uniform.location), false, &uniform_values[name]
                    );
                },
            }
        }

        updated
    }

    fn select_shader(
        &self,
        renderer: &GlRenderer
    ) -> ShaderProgram {
        let geo = self.get_geometry();
        let gl = &renderer.gl;

        let program = if geo.positions.is_some() && geo.normals.is_some() {
            renderer.programs[&ShaderProgramType::PosAndNormal].clone()
        }
        else if geo.positions.is_some() && geo.colors.is_some() {
            renderer.programs[&ShaderProgramType::PosAndColor].clone()
        }
        else {
            renderer.programs[&ShaderProgramType::PosOnly].clone()
        }; 
            
        unsafe { 
            gl.use_program(Some(program.program)); 
        }

        program
    }

    pub fn draw(
        &mut self,
        world_matrix: Option<&Matrix4>,
        camera: &dyn ObjectCamera,
        renderer: &GlRenderer
    ) {
        unsafe {
            let program = self.select_shader(renderer);

            let updated = self.update(
                world_matrix, 
                camera, 
                &program,
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

