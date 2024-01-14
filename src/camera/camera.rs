use crate::math::Matrix4;

pub struct Camera {
    pub projection_matrix: Matrix4,
    pub projection_matrix_inverse: Matrix4,
}

impl Camera {
    pub fn new(
    ) -> Self {
        Self {
            projection_matrix: Matrix4::identity(),
            projection_matrix_inverse: Matrix4::identity(),
        }
    }
}