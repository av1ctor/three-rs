use std::{rc::Rc, cell::RefCell};
use glow::*;
use crate::{core::RenderableObject, renderer::GlRenderer};

pub struct Scene {
    pub renderer: Rc<RefCell<GlRenderer>>,
    pub objects: Vec<Rc<RefCell<dyn RenderableObject>>>,
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
    ) where T: RenderableObject + 'static {
        self.objects.push(obj);
    }

    pub fn render(
        &mut self,
    ) {
        let renderer = &self.renderer.borrow_mut();
        
        unsafe {
            renderer.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            for object in &mut self.objects {
                let obj = &mut object.borrow_mut();
                if obj.get_base().visible {
                    obj.render(renderer);
                }
            }

            renderer.window.gl_swap_window();
        }
    }
}