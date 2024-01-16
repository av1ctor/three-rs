use std::{cell::RefCell, rc::Rc};

use crate::{math::Matrix4, core::{Object3d, Objectifiable, GeometricalRenderable, Transformable}};

pub struct Camera {
    pub obj: Object3d,
    pub proj_matrix: Matrix4,
    pub proj_matrix_inverse: Matrix4,
}

impl Camera {
    pub fn new(
    ) -> Self {
        Self {
            obj: Object3d::new(),
            proj_matrix: Matrix4::identity(),
            proj_matrix_inverse: Matrix4::identity(),
        }
    }
}

impl Objectifiable for Camera {
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

    fn add(
        &mut self,
        child: Rc<RefCell<dyn GeometricalRenderable>>
    ) {
        self.obj.add(child);
    }
}

impl Transformable for Camera {
}