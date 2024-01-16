use crate::{math::Matrix4, core::{Object3d, Objectifiable}};

pub struct Camera {
    pub obj: Object3d,
    pub projection_matrix: Matrix4,
    pub projection_matrix_inverse: Matrix4,
}

impl Camera {
    pub fn new(
    ) -> Self {
        Self {
            obj: Object3d::new(),
            projection_matrix: Matrix4::identity(),
            projection_matrix_inverse: Matrix4::identity(),
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
}