use serde::{Serialize, Deserialize};
use super::Vector3;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vector3,
    pub constant: f32,
}

impl Plane {
    pub fn from_coplanar_points(
        a: &Vector3,
        b: &Vector3,
        c: &Vector3
    ) -> Self {
        let normal = c.sub(b).cross(&a.sub(b)).normalize();
		let constant = -a.dot(&normal);

        Self {
            normal,
            constant,
        }
    }

    pub fn distance_to_point(
        &self,
        point: &Vector3 
    ) -> f32 {
		self.normal.dot(point) + self.constant
	}
}