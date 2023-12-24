use serde::{Serialize, Deserialize};
use super::{vector3::Vector3, plane::Plane};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Triangle {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}

impl Triangle {
    pub fn new(
        a: Vector3,
        b: Vector3,
        c: Vector3
    ) -> Self {
        Self {
            a,
            b,
            c,
        }
    }

    pub fn get_plane(
        &self
    ) -> Plane {
		Plane::from_coplanar_points(&self.a, &self.b, &self.c)
	}

    pub fn get_barycoord(
        &self,
        point: &Vector3 
    ) -> Option<Vector3> {

		let v0 = self.c.sub(&self.a);
		let v1 = self.b.sub(&self.a);
		let v2 = point.sub(&self.a);

		let dot00 = v0.dot(&v0);
		let dot01 = v0.dot(&v1);
		let dot02 = v0.dot(&v2);
		let dot11 = v1.dot(&v1);
		let dot12 = v1.dot(&v2);

		let denom = dot00 * dot11 - dot01 * dot01;
		if denom == 0.0 {
			return Some(Vector3::zero());
		}

		let inv_denom = 1.0 / denom;
		let u = ( dot11 * dot02 - dot01 * dot12 ) * inv_denom;
		let v = ( dot00 * dot12 - dot01 * dot02 ) * inv_denom;

		Some(Vector3::new( 1.0 - u - v, v, u))
	}

	pub fn contains_point(
        &self,
        point: &Vector3
    ) -> bool {
		if let Some(_v3) = self.get_barycoord(point) {
		    (_v3.x >= 0.0) && (_v3.y >= 0.0) && ((_v3.x + _v3.y) <= 1.0)
        }
        else {
            false
        }
	}
}