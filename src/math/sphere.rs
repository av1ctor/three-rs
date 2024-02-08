use serde::{Serialize, Deserialize};
use super::Vector3;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(
        center: Vector3,
        radius: f32
    ) -> Self {
        Self {
            center,
            radius,
        }
    }

    pub fn empty(
    ) -> Self {
        Self {
            center: Vector3::zero(),
            radius: f32::MIN,
        }
    }

    pub fn is_empty(
        &self
    ) -> bool {
        self.radius == f32::MIN
    }

    pub fn contains_point(
        &self,
        point: &Vector3
    ) -> bool {
		point.distance_to_sq(&self.center) <= (self.radius * self.radius)
	}

	pub fn distance_to_point(
        &self,
        point: &Vector3
    ) -> f32 {
		point.distance_to(&self.center) - self.radius
	}

	pub fn intersects_sphere(
        &self,
        sphere: Self
    ) -> bool {
		let radius_sum = self.radius + sphere.radius;
		sphere.center.distance_to_sq(&self.center) <= (radius_sum * radius_sum)
	}
}
