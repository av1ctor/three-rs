use std::f32::consts::PI;

use crate::math::Matrix4;

use super::Camera;

pub struct PerspectiveCamera {
    pub camera: Camera,
    pub fov: f32,
    pub zoom: f32,
    pub near: f32,
    pub far: f32,
    pub focus: f32,
    pub aspect: f32,
    pub film_gauge: f32,
    pub film_offset: f32,
}

impl PerspectiveCamera {
    pub fn new(
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32
    ) -> Self {
        let mut cam = Self {
            camera: Camera::new(),
            fov,
            zoom: 1.0,
            near,
            far,
            focus: 10.0,
            aspect,
            film_gauge: 35.0,
            film_offset: 0.0,
        };

        cam.update_projection_matrix();

        cam
    }

    pub fn get_film_width(
        &self
    ) -> f32 {
		self.film_gauge * self.aspect.min(1.0)
	}

	pub fn get_film_height(
        &self
    ) -> f32 {
        self.film_gauge / self.aspect.max(1.0)
	}

    fn update_projection_matrix(
        &mut self
    ) {
        let near = self.near;
		let top = near * (0.5 * self.fov * PI / 180.0).tan() / self.zoom;
		let height = 2.0 * top;
		let width = self.aspect * height;
		let mut left = - 0.5 * width;

		let skew = self.film_offset;
		if skew != 0.0 {
            left += near * skew / self.get_film_width();
        };

        self.camera.proj_matrix = Matrix4::perspective(
            left, left + width, top, top - height, near, self.far
        );

		self.camera.proj_matrix_inverse = self.camera.proj_matrix.invert();
    }
}