use std::{rc::Rc, cell::RefCell};

use crate::{
    core::{
        ObjectData, 
        BufferGeometry, 
        Object3d, 
        Geometrical, 
        Renderable, 
        GeometricalRenderable, 
        Transformable
    }, 
    renderer::GlRenderer, 
    math::Matrix4, 
    camera::ObjectCamera
};

#[derive(Clone)]
pub struct Mesh {
    obj: ObjectData,
    geo: BufferGeometry,
}

impl Mesh {
    pub fn new(
        geo: &dyn Geometrical
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            obj: ObjectData::new(),
            geo: geo.get_geometry().clone(),
        }))
    }
}

impl Object3d for Mesh {
    fn get_object(
        &self
    ) -> &ObjectData {
        &self.obj
    }

    fn get_object_mut(
        &mut self
    ) -> &mut ObjectData {
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
        camera: &dyn ObjectCamera,
        renderer: &GlRenderer
    ) {
        (self as &mut dyn Renderable).draw(
            world_matrix, 
            camera,
            renderer
        )
    }
}

impl GeometricalRenderable for Mesh {
}

impl Transformable for Mesh {
}