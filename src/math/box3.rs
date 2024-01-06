use serde::{Serialize, Deserialize};
use super::{vector3::Vector3, triangle::Triangle};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Box3 {
    pub min: Vector3,
    pub max: Vector3,
}

impl Default for Box3 {
    fn default() -> Self {
        Self { 
            min: Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY), 
            max: Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY)
        }
    }
}

impl Box3 {
    pub fn new(
        min: Vector3,
        max: Vector3
    ) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn is_empty(
        &self
    ) -> bool {
		(self.max.x < self.min.x) || 
        (self.max.y < self.min.y) || 
        (self.max.z < self.min.z)
	}

    pub fn get_center( 
        &self
    ) -> Vector3 {
		if self.is_empty() {
            Vector3::zero()
        } 
        else {
            self.min.add(&self.max).mul_scalar(0.5)
        }
	}

    pub fn intersects_with_box(
        &self,
        bx: &Self
    ) -> bool {
        if bx.max.x < self.min.x || bx.min.x > self.max.x ||
			bx.max.y < self.min.y || bx.min.y > self.max.y ||
			bx.max.z < self.min.z || bx.min.z > self.max.z {
            false
        }
        else {
            true
        }
    }

    pub fn intersects_with_triangle(
        &self,
        tri: &Triangle
    ) -> bool {
        if self.is_empty() {
            return false;
        }

        let center = self.get_center();
        let extents = self.max.sub(&center);

		let v0 = tri.a.sub(&center);
		let v1 = tri.b.sub(&center);
		let v2 = tri.c.sub(&center);

		let f0 = v1.sub(&v0);
		let f1 = v2.sub(&v1);
		let f2 = v0.sub(&v2);

        if !Self::sat_for_axes(
            &[
                0.0, -f0.z, f0.y, 0.0, -f1.z, f1.y, 0.0, -f2.z, f2.y,
                f0.z, 0.0, -f0.x, f1.z, 0.0, -f1.x, f2.z, 0.0, -f2.x,
                -f0.y, f0.x, 0.0, -f1.y, f1.x, 0.0, -f2.y, f2.x, 0.0
            ], 
            &v0, &v1, &v2, &extents) {
			return false;
		}

		if !Self::sat_for_axes(
            &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], 
            &v0, &v1, &v2, &extents){
			return false;
		}
        
		let normal = f0.cross(&f1);
		return Self::sat_for_axes(
            &[normal.x, normal.y, normal.z], 
            &v0, &v1, &v2, &extents
        );
    }

    fn sat_for_axes(
        axes: &[f32], 
        v0: &Vector3, 
        v1: &Vector3, 
        v2: &Vector3, 
        extents: &Vector3 
    ) -> bool {

        let mut i = 0;
        let j = axes.len() - 3;
        while i <= j {
            let test = Vector3::from_array(&axes, i);
            
            let r = extents.x * test.x.abs() + extents.y * test.y.abs() + extents.z * test.z.abs();

            let p0 = v0.dot(&test);
            let p1 = v1.dot(&test);
            let p2 = v2.dot(&test);
            if f32::max(-f32::max(f32::max(p0, p1), p2), f32::min(f32::min(p0, p1), p2)) > r {
                return false;
            }

            i += 3;
        }
    
        true
    }
}