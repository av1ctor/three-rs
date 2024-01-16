use crate::{
    math::Matrix4, 
    core::{Object3d, ObjectData, Updatable, Transformable}
};
use super::{CameraData, ObjectCamera, Camera};

pub struct OrthographicCamera {
    pub(crate) cam: CameraData,
    pub zoom: f32,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

impl OrthographicCamera {
    pub fn new(
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32
    ) -> Self {
        let mut cam = Self {
            cam: CameraData::new(),
            near,
            far,
            zoom: 1.0,
            left,
            right,
            top,
            bottom,
        };

        cam.update_projection_matrix();

        cam
    }

    fn update_projection_matrix(
        &mut self
    ) {
        let dx = (self.right - self.left) / (2.0 * self.zoom);
		let dy = (self.top - self.bottom) / (2.0 * self.zoom);
		let cx = (self.right + self.left) / 2.0;
		let cy = (self.top + self.bottom) / 2.0;

		let left = cx - dx;
		let right = cx + dx;
		let top = cy + dy;
		let bottom = cy - dy;

        self.cam.proj_matrix = Matrix4::orthographic(
            left, right, top, bottom, self.near, self.far
        );

		self.cam.proj_matrix_inverse = self.cam.proj_matrix.invert();
    }
}

impl Object3d for OrthographicCamera {
    fn get_object(
        &self
    ) -> &ObjectData {
        &self.cam.obj
    }

    fn get_object_mut(
        &mut self
    ) -> &mut ObjectData {
        &mut self.cam.obj
    }
}

impl Transformable for OrthographicCamera {
}

impl Updatable for OrthographicCamera {
    fn update_matrix(
        &mut self,
    ) {
        self.cam.obj.update_matrix();
        self.cam.world_matrix_inverse = self.cam.obj.matrix.invert();
    }
}

impl Camera for OrthographicCamera {
    fn get_data(
        &self
    ) -> &CameraData {
        &self.cam
    }

    fn get_data_mut(
        &mut self
    ) -> &mut CameraData {
        &mut self.cam
    }
}

impl ObjectCamera for OrthographicCamera {
}