use glow::*;
use crate::math::{Matrix4, {Vector3, UP}};

const VERTEX_SHADER_SOURCE: &str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("../shaders/frag.glsl");

pub struct ShaderUniformLocations {
    pub projection: UniformLocation,
    pub view: UniformLocation,
    pub model: UniformLocation,
}

pub struct GlRenderer {
    pub gl: Context,
    pub window: sdl2::video::Window,
    pub events_loop: sdl2::EventPump,
    pub context: sdl2::video::GLContext,
    pub attrib_locations: (u32, u32),
    pub uniform_locations: ShaderUniformLocations,
    pub program: NativeProgram,
}

impl Drop for GlRenderer {
    fn drop(
        &mut self
    ) {
        unsafe {
            let gl = &self.gl;
            
            gl.delete_program(self.program);
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
        let context = window.gl_create_context().unwrap();
        let gl =
            Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        let events_loop = sdl.event_pump().unwrap();

        let program = Self::create_program(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
        gl.use_program(Some(program));

        let attrib_locations = (0, 1);

        let uniform_locations = Self::configure_gl(&gl, w, h, &program);

        Self {
            gl,
            window,
            events_loop,
            context,
            attrib_locations,
            uniform_locations,
            program
        }

    }

    pub fn poll_events(
        &mut self
    ) -> sdl2::event::EventPollIterator {
        self.events_loop.poll_iter()
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

    unsafe fn configure_gl(
        gl: &Context,
        w: u32,
        h: u32,
        program: &NativeProgram
    ) -> ShaderUniformLocations {
        let projection_loc = gl.get_uniform_location(*program, "projection").unwrap();
        let view_loc = gl.get_uniform_location(*program, "view").unwrap();
        let model_loc = gl.get_uniform_location(*program, "model").unwrap();

        let projection = Matrix4::perspective_fov(45.0, (w as f32) / (h as f32), 0.1, 1000.0);
        gl.uniform_matrix_4_f32_slice(Some(&projection_loc), false, projection.to_slice());

        let view = Matrix4::look_at(
            &Vector3::new(0.0, 10.0, 50.0), 
            &Vector3::new(0.0, 0.0, 0.0), 
            &UP
        );
        gl.uniform_matrix_4_f32_slice(Some(&view_loc), false, view.to_slice());
        
        gl.viewport(0, 0, w as _, h as _);
        gl.enable(DEPTH);
        gl.enable(COLOR);
        gl.enable(CULL_FACE);
        gl.clear_color(0.5, 0.5, 0.5, 1.0);

        ShaderUniformLocations {
            projection: projection_loc,
            view: view_loc,
            model: model_loc,
        }
   }
}