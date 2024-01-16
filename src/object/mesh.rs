use std::{rc::Rc, cell::RefCell};

use crate::{
    core::{
        Object3d, 
        BufferGeometry, 
        Objectifiable, 
        Geometrical, 
        Renderable, 
        GeometricalRenderable, 
        Transformable
    }, 
    renderer::GlRenderer, math::Matrix4
};

pub struct Mesh {
    obj: Object3d,
    geo: BufferGeometry,
}

impl Mesh {
    pub fn new(
        geo: &dyn Geometrical
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            obj: Object3d::new(),
            geo: geo.get_geometry().clone(),
        }))
    }
}

impl Objectifiable for Mesh {
    fn get_object(
        &self
    ) -> &Object3d {
        &self.obj
    }

    fn get_object_mut(
        &mut self
    ) -> &mut Object3d {
        &mut self.obj
    }
}

impl Geometrical for Mesh {
    fn get_geometry(
        &self
    ) -> &BufferGeometry {
        &self.geo
    }

    fn get_geometry_mut(
        &mut self
    ) -> &mut BufferGeometry {
        &mut self.geo
    }

    fn drop(
        &mut self, 
        renderer: &GlRenderer
    ) {
        (self as &mut dyn Geometrical).destroy(renderer)
    }
}

impl Renderable for Mesh {
    fn render(
        &mut self, 
        world_matrix: Option<&Matrix4>,
        renderer: &GlRenderer
    ) {
        (self as &mut dyn Renderable).draw(world_matrix, renderer)
    }
}

impl GeometricalRenderable for Mesh {
}

impl Transformable for Mesh {
}