use std::{rc::Rc, cell::RefCell};
use glow::*;
use crate::{core::GeometricalRenderable, renderer::GlRenderer, camera::PerspectiveCamera};

pub struct Scene {
    pub renderer: Rc<RefCell<GlRenderer>>,
    pub objects: Vec<Rc<RefCell<dyn GeometricalRenderable>>>,
}

impl Drop for Scene {
    fn drop(
        &mut self
    ) {
        let renderer = &self.renderer.borrow_mut();
        for obj in &mut self.objects {
            obj.borrow_mut().drop(renderer);
        }
    }
}

impl Scene {
    pub fn new(
        renderer: Rc<RefCell<GlRenderer>>
    ) -> Self {
        Self {
            renderer,
            objects: vec![],
        }
    }

    pub fn add<T>(
        &mut self,
        obj: Rc<RefCell<T>>
    ) where T: GeometricalRenderable + 'static {
        self.objects.push(obj);
    }

    pub fn render(
        &mut self,
        camera: &PerspectiveCamera
    ) {
        let renderer = &self.renderer.borrow_mut();
        let gl = &renderer.gl;
        
        unsafe {
            gl.uniform_matrix_4_f32_slice(
                Some(&renderer.uniform_locations.projection), 
                false, 
                camera.camera.projection_matrix.to_slice()
            );
    
            renderer.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            for object in &mut self.objects {
                let obj = &mut object.borrow_mut();
                if obj.get_object().visible {
                    obj.render(None, renderer);
                }
            }

            renderer.window.gl_swap_window();
        }
    }
}