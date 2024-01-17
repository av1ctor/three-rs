use std::{rc::Rc, cell::RefCell};
use glow::*;
use crate::{
    core::GeometricalRenderable, 
    renderer::GlRenderer, 
    camera::ObjectCamera,
};

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

    pub fn reset(
        &mut self
    ) {
        let renderer = &self.renderer.borrow_mut();
        for obj in &mut self.objects {
            obj.borrow_mut().drop(renderer);
        }
        self.objects = vec![];
    }

    pub fn add<T>(
        &mut self,
        obj: Rc<RefCell<T>>
    ) where T: GeometricalRenderable + 'static {
        self.objects.push(obj);
    }

    pub fn render(
        &mut self,
        camera: &mut dyn ObjectCamera
    ) {
        camera.update_matrix();
        
        let renderer = &self.renderer.borrow_mut();
        
        unsafe {
            renderer.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            for object in &mut self.objects {
                let obj = &mut object.borrow_mut();
                if obj.get_object().visible {
                    obj.render(None, camera, renderer);
                }
            }

            renderer.window.gl_swap_window();
        }
    }
}