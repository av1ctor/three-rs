use serde::{Deserialize, Serialize};
use super::{
    vector3::Vector3, 
    box3::Box3, 
    triangle::Triangle, 
    line3::Line3, 
    misc::line_to_line_closest_points
};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Capsule {
    pub start: Vector3,
    pub end: Vector3,
    pub radius: f32,
}

impl Default for Capsule {
    fn default(
    ) -> Self {
        Self {
            start: Vector3::zero(),
            end: Vector3::new(0.0, 1.0, 0.0),
            radius: 1.0,
        }
    }
}

impl Capsule {
    pub fn new(
        start: Vector3,
        end: Vector3,
        radius: f32
    ) -> Self {
        Self {
            start,
            end,
            radius,
        }
    }

    pub fn get_center( 
        &self
    ) -> Vector3 {
		self.end.add(&self.start).mul_scalar(0.5)
	}

    pub fn translate(
        &self,
        v: &Vector3
    ) -> Self {
        Self {
            start: self.start.add(v),
            end: self.start.add(v),
            radius: self.radius,
        }
	}

    fn check_aabb_axis(
        p1x: f32, 
        p1y: f32, 
        p2x: f32, 
        p2y: f32, 
        minx: f32, 
        maxx: f32, 
        miny: f32, 
        maxy: f32, 
        radius: f32
    ) -> bool {
        (minx - p1x < radius || minx - p2x < radius) &&
        (p1x - maxx < radius || p2x - maxx < radius) &&
        (miny - p1y < radius || miny - p2y < radius) &&
        (p1y - maxy < radius || p2y - maxy < radius)
	}

	pub fn intersects_box(
        &self,
        bx: &Box3 
    ) -> bool {
        Self::check_aabb_axis(
            self.start.x, self.start.y, self.end.x, self.end.y,
            bx.min.x, bx.max.x, bx.min.y, bx.max.y,
            self.radius) &&
        Self::check_aabb_axis(
            self.start.x, self.start.z, self.end.x, self.end.z,
            bx.min.x, bx.max.x, bx.min.z, bx.max.z,
            self.radius) &&
        Self::check_aabb_axis(
            self.start.y, self.start.z, self.end.y, self.end.z,
            bx.min.y, bx.max.y, bx.min.z, bx.max.z,
            self.radius)
	}

    pub fn intersecting_triangle( 
        &self, 
        triangle: &Triangle
    ) -> Option<(Vector3, Vector3, f32)> {
		let plane = triangle.get_plane();

		let d1 = plane.distance_to_point(&self.start) - self.radius;
		let d2 = plane.distance_to_point(&self.end) - self.radius;

		if (d1 > 0.0 && d2 > 0.0) || (d1 < - self.radius && d2 < - self.radius) {
			return None;
		}

		let delta = f32::abs(d1 / (f32::abs(d1) + f32::abs(d2)));
		let intersect_point = self.start.lerp(&self.end, delta);

		if triangle.contains_point(&intersect_point) {
			return Some((
                plane.normal, 
                intersect_point, 
                f32::abs(f32::min( d1, d2 ))
            ));
		}

		let r2 = self.radius * self.radius;

		let line1 = Line3::new(self.start, self.end);

		let lines = [
			Line3::new(triangle.a, triangle.b),
			Line3::new(triangle.b, triangle.c),
            Line3::new(triangle.c, triangle.a)
		];

		for line2 in lines {
			let (point1, point2) = line_to_line_closest_points(&line1, &line2);

			if point1.distance_to_sq(&point2) < r2 {
				return Some((
					point1.sub(&point2).normalize(),
					point2,
					self.radius - point1.distance_to(&point2)
                ));
			}
		}

		None
	}
}