use std::collections::HashMap;
use glow::*;
use crate::math::Matrix4;
use super::Event;
use crate::core::BufferGeometry;

#[derive(Clone, Debug)]
#[repr(C)]
#[allow(dead_code)]
pub(crate) enum ShaderUniformType {
    Vector3,
    Matrix4,
}

#[derive(Clone, Debug)]
pub(crate) struct ShaderUniform {
    pub ty: ShaderUniformType,
    pub location: UniformLocation,
}

#[derive(Clone, Debug)]
pub(crate) struct ShaderUniformLocations {
    pub projection: UniformLocation,
    pub model_view: UniformLocation,
    pub other: HashMap<String, ShaderUniform>,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ShaderProgramType {
    PosOnly = 0,
    PosAndNormal,
    PosAndColor,
}

#[derive(Clone, Debug)]
pub(crate) struct ShaderProgram {
    pub program: NativeProgram,
    pub uniform_locations: ShaderUniformLocations,
}

const SHADER_SOURCES: &[(ShaderProgramType, &str, &str, &[(&str, ShaderUniformType)])] = &[
    (
        ShaderProgramType::PosOnly,
        include_str!("../shaders/pos/vertex.glsl"), 
        include_str!("../shaders/pos/frag.glsl"),
        &[("color", ShaderUniformType::Vector3)],
    ),
    (
        ShaderProgramType::PosAndNormal,
        include_str!("../shaders/pos_normal/vertex.glsl"), 
        include_str!("../shaders/pos_normal/frag.glsl"),
        &[],
    ),
    (
        ShaderProgramType::PosAndColor,
        include_str!("../shaders/pos_color/vertex.glsl"), 
        include_str!("../shaders/pos_color/frag.glsl"),
        &[],
    )
];

pub struct GlRenderer {
    pub(crate) gl: Context,
    window: sdl2::video::Window,
    events_loop: sdl2::EventPump,
    _context: sdl2::video::GLContext,
    pub(crate) programs: HashMap<ShaderProgramType, ShaderProgram>,
}

impl Drop for GlRenderer {
    fn drop(
        &mut self
    ) {
        unsafe {
            let gl = &self.gl;
            
            for program in self.programs.values() {
                gl.delete_program(program.program);
            }
        }
    }
}

impl GlRenderer {
    pub unsafe fn new(
        title: &str,
        w: u32,
        h: u32
    ) -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        
        let window = video
            .window(title, w, h)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        
        // NOTE: if removed the GL initialization will crash
        let _context = window.gl_create_context().unwrap();
        
        let gl =
            Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        
        let events_loop = sdl.event_pump().unwrap();

        Self::configure_gl(&gl, w, h);

        let mut programs = HashMap::default();

        for source in SHADER_SOURCES {
            let program = Self::create_program(&gl, source.1, source.2);
            let uniform_locations = Self::get_uniform_locations(
                &gl, &program, source.3
            );
            programs.insert(
                source.0.clone(), 
                ShaderProgram{
                    program,
                    uniform_locations,
                }
            );
        }

        Self {
            gl,
            window,
            events_loop,
            _context,
            programs
        }
    }

    pub fn poll_events(
        &mut self
    ) -> Vec<Event> {
        let mut events = vec![];
        
        for event in self.events_loop.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    events.push(Event::Quit);
                },
                sdl2::event::Event::KeyDown {scancode: Some(scancode), ..} => {
                    events.push(Event::KeyDown(scancode as _));
                }
                _ => {}
            }
        }
        
        events
    }

    pub fn swap_window(
        &self
    ) {
        self.window.gl_swap_window();
    }

    pub fn clear(
        &self
    ) {
        unsafe {
            self.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT)
        }
    }

    pub fn create_buffers(
        &self,
        geo: &mut BufferGeometry
    ) {
        let gl = &self.gl;

        unsafe {
            geo.vbo = Some(gl.create_buffer().unwrap());
            geo.ebo = Some(gl.create_buffer().unwrap());
            geo.vao = Some(gl.create_vertex_array().unwrap());
        }
    }

    pub fn delete_buffers(
        &self,
        geo: &BufferGeometry
    ) {
        let gl = &self.gl;

        unsafe {
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

    unsafe fn create_program(
        gl: &glow::Context,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> NativeProgram {
        let program = gl.create_program().expect("Cannot create program");
    
        let shader_sources = [
            (VERTEX_SHADER, vertex_shader_source),
            (FRAGMENT_SHADER, fragment_shader_source),
        ];
    
        let mut shaders = Vec::with_capacity(shader_sources.len());
    
        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, shader_source);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
    
        program
    }

    unsafe fn get_uniform_locations(
        gl: &Context,
        program: &NativeProgram,
        uniforms: &[(&str, ShaderUniformType)]

    ) -> ShaderUniformLocations {
        // find uniforms present in all shaders
        let projection_loc = gl.get_uniform_location(*program, "projection").unwrap();
        let proj = Matrix4::identity();
        gl.uniform_matrix_4_f32_slice(Some(&projection_loc), false, proj.to_slice());

        let model_view_loc = gl.get_uniform_location(*program, "model_view").unwrap();
        let view = Matrix4::identity();
        gl.uniform_matrix_4_f32_slice(Some(&model_view_loc), false, view.to_slice());

        // find shader-specific uniforms
        let mut other = HashMap::default();
        for uni in uniforms {
            let location = gl.get_uniform_location(*program, uni.0).unwrap();
            other.insert(uni.0.to_string(), ShaderUniform{
                ty: uni.1.clone(),
                location,
            });
        }
        
        ShaderUniformLocations {
            projection: projection_loc,
            model_view: model_view_loc,
            other
        }
    }

    unsafe fn configure_gl(
        gl: &Context,
        w: u32,
        h: u32
    ) {
        gl.viewport(0, 0, w as _, h as _);
        gl.enable(DEPTH_TEST);
        gl.enable(COLOR);
        gl.enable(CULL_FACE);
        gl.enable(MULTISAMPLE);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
   }
}