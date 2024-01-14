use super::{Vector3, Box3, Triangle};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Default for Ray {
    fn default(
    ) -> Self {
        Self { 
            origin: Vector3::zero(), 
            direction: Vector3::new(0.0, 0.0, -1.0),
        }
    }
}

impl Ray {
    pub fn new(
        origin: Vector3,
        direction: Vector3,
    ) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn at(
        &self,
        scale: f32
    ) -> Vector3 {
		self.origin.add(&self.direction).mul_scalar(scale)
	}

	pub fn look_at(
        &mut self,
        v: &Vector3
    ) -> &Self {
		self.direction = v.sub(&self.origin).normalize();
        self
	}

    pub fn intersecting_box(
        &self,
        bx: &Box3
    ) -> Option<Vector3> {
		let invdirx = 1.0 / self.direction.x;
		let invdiry = 1.0 / self.direction.y;
		let invdirz = 1.0 / self.direction.z;

        let origin = self.origin;

		let (mut tmin, mut tmax) = if invdirx >= 0.0 {
			(
                (bx.min.x - origin.x) * invdirx,
			    (bx.max.x - origin.x) * invdirx
            )
		} 
        else {
            (
                (bx.max.x - origin.x) * invdirx,
			    (bx.min.x - origin.x) * invdirx
            )
		};

		let (tymin, tymax) = if invdiry >= 0.0 {
			(
                (bx.min.y - origin.y) * invdiry,
			    (bx.max.y - origin.y) * invdiry
            )
		} 
        else {
			(
                (bx.max.y - origin.y) * invdiry,
			    (bx.min.y - origin.y) * invdiry
            )
		};

		if (tmin > tymax) || (tymin > tmax) {
            return None;
        }

		if tymin > tmin || f32::is_nan(tmin) {
            tmin = tymin
        };

		if tymax < tmax || f32::is_nan(tmax) {
            tmax = tymax
        };

		let (tzmin, tzmax) = if invdirz >= 0.0 {
            (
			    (bx.min.z - origin.z) * invdirz,
			    (bx.max.z - origin.z) * invdirz
            )
		} 
        else {
			(
                (bx.max.z - origin.z) * invdirz,
			    (bx.min.z - origin.z) * invdirz
            )
		};

		if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }

		if tzmin > tmin || tmin != tmin {
            tmin = tzmin
        };

		if tzmax < tmax || tmax != tmax {
            tmax = tzmax
        };

		if tmax < 0.0 {
            None
        }
        else {
		    Some(self.at(if tmin >= 0.0 {tmin} else {tmax}))
        }
	}

    pub fn intersects_box(
        &self,
        bx: &Box3
    ) -> bool {
        if self.intersecting_box(bx).is_some() {
            true
        } 
        else {
            false
        }
    }

    pub fn intersecting_triangle(
        &self,
        tri: &Triangle, 
        backface_culling: bool 
    ) -> Option<Vector3> {
		let edge1 = tri.b.sub(&tri.a);
		let edge2 = tri.c.sub(&tri.a);
		let normal = edge1.cross(&edge2);

		let mut dqn = self.direction.dot(&normal);
		let sign = if dqn > 0.0 {
			if backface_culling {
                return None;
            };
			1.0
		} 
        else if dqn < 0.0 {
			dqn = -dqn;
            -1.0
		} 
        else {
			return None;
		};

		let diff = self.origin.sub(&tri.a);
		let ddqxe2 = sign * self.direction.dot(&diff.cross(&edge2));
		if ddqxe2 < 0.0 {
			return None;
		}

		let dde1xq = sign * self.direction.dot(&edge1.cross(&diff));
		if dde1xq < 0.0 {
			return None;
		}

		if ddqxe2 + dde1xq > dqn {
			return None;
		}

		let qdn = -sign * diff.dot(&normal);
		if qdn < 0.0 {
			None
		}
        else {
		    Some(self.at(qdn / dqn))
        }
	}
}