use serde::{Serialize, Deserialize};
use super::{matrix4::Matrix4, quaternion::Quaternion};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vector3 {
    fn default(
    ) -> Self {
        Self::zero()
    }
}

impl Vector3 {
    pub fn new(
        x: f32,
        y: f32,
        z: f32
    ) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn from_slice(
        src: &[f32; 3]
    ) -> Self {
        Self {
            x: src[0],
            y: src[1],
            z: src[2],
        }
    }

    pub fn from_array(
        src: &[f32],
        i: usize
    ) -> Self {
        Self {
            x: src[i],
            y: src[i+1],
            z: src[i+2],
        }
    }

    pub fn to_slice(
        &self
    ) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn zero(
    ) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one(
    ) -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn add(
        &self,
        other: &Self
    ) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn add_mut(
        &mut self,
        other: &Self
    ) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn sub(
        &self,
        other: &Self
    ) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn sub_scalar(
        &self,
        scalar: f32
    ) -> Self {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar,
        }
    }

    pub fn mul(
        &self,
        other: &Self
    ) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn mul_scalar(
        &self,
        scalar: f32
    ) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn div(
        &self,
        v: &Vector3
    ) -> Self {
		Self {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
	}

    pub fn div_scalar(
        &self,
        scalar: f32
    ) -> Self {
        self.mul_scalar(1.0 / scalar)
    }

    pub fn dot( 
        &self,
        other: &Self
    ) -> f32 {
            self.x * other.x +
            self.y * other.y +
            self.z * other.z
	}

    pub fn cross( 
        &self,
        other: &Self
    ) -> Self {
		Self {
            x: self.y * other.z - self.z * other.y,
		    y: self.z * other.x - self.x * other.z,
		    z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_sq(
        &self
    ) -> f32 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

    pub fn length(
        &self
    ) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

    pub fn normalize(
        &self
    ) -> Self {
		let len = self.length();
        self.div_scalar(if len != 0.0 {len} else {1.0})
	}

    pub fn distance_to(
        &self,
        v: &Vector3
    ) -> f32 {
		f32::sqrt(self.distance_to_sq(v))
	}

	pub fn distance_to_sq(
        &self,
        v: &Vector3
    ) -> f32 {
		let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;
		dx * dx + dy * dy + dz * dz
	}

    pub fn lerp(
        &self,
        v: &Vector3,
        alpha: f32
    ) -> Self {
        Self {
            x: self.x + (v.x - self.x) * alpha,
            y: self.y + (v.y - self.y) * alpha,
            z: self.z + (v.z - self.z) * alpha,
        }
	}

    pub fn apply_matrix4( 
        &self,
        m: &Matrix4
    ) -> Self {

		let x = self.x; 
        let y = self.y;
        let z = self.z;
		let e = &m.0;

		let w = 1.0 / (e[0][3] * x + e[1][3] * y + e[2][3] * z + e[3][3]);

		Self {
            x: (e[0][0] * x + e[1][0] * y + e[2][0] * z + e[3][0]) * w,
		    y: (e[0][1] * x + e[1][1] * y + e[2][1] * z + e[3][1]) * w,
		    z: (e[0][2] * x + e[1][2] * y + e[2][2] * z + e[3][2]) * w,
        }
    }

    pub fn apply_quaternion( 
        &self,
        q: &Quaternion
    ) -> Self {
		// t = 2 * cross( q.xyz, v );
		let tx = 2.0 * (q.y * self.z - q.z * self.y);
		let ty = 2.0 * (q.z * self.x - q.x * self.z);
		let tz = 2.0 * (q.x * self.y - q.y * self.x);

		// v + q.w * t + cross( q.xyz, t );
        Self {
            x: self.x + q.w * tx + q.y * tz - q.z * ty,
		    y: self.y + q.w * ty + q.z * tx - q.x * tz,
		    z: self.z + q.w * tz + q.x * ty - q.y * tx,
        }
    }

}