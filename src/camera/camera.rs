use crate::{math::Matrix4, core::Object3d};

pub struct Camera {
    pub obj: Object3d,
    pub proj_matrix: Matrix4,
    pub proj_matrix_inverse: Matrix4,
    pub world_matrix_inverse: Matrix4,
}

impl Camera {
    pub fn new(
    ) -> Self {
        Self {
            obj: Object3d::new(),
            proj_matrix: Matrix4::identity(),
            proj_matrix_inverse: Matrix4::identity(),
            world_matrix_inverse: Matrix4::identity(),
        }
    }
}