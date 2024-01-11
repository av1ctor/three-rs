use std::f32::consts::PI;
use serde::{Serialize, Deserialize};
use super::{matrix4::Matrix4, quaternion::Quaternion, euler::Euler};

pub const RIGHT: Vector3 = Vector3{x: 1.0, y: 0.0, z: 0.0};
pub const UP: Vector3 = Vector3{x: 0.0, y: 1.0, z: 0.0};
pub const FORWARD: Vector3 = Vector3{x: 0.0, y: 0.0, z: 1.0};

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

    pub fn from_euler(
        euler: &Euler
    ) -> Self {
        Self {
            x: euler.v.x,
            y: euler.v.y,
            z: euler.v.z,
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

    pub fn neg(
        &self
    ) -> Self {
        Self {
            x: -self.x, 
            y: -self.y,
            z: -self.z,
        }
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

    pub fn hypot2(
        &self,
        other: &Self
    ) -> f32 {
        let s = self.sub(other);
        s.dot(&s)
    }

    pub fn project(
        &self,
        other: &Self
    ) -> Self {
        let d = other.length_sq();
		if d == 0.0 {
            return Vector3::zero()
        };

		let k = other.dot(&self) / d;
        
        other.mul_scalar(k)
    }

    pub fn project_on_plane(
        &self,
        plane_normal: &Self
    ) -> Self {
        self.sub(&self.project(plane_normal))
    }

    pub fn reflect(
        &self,
        normal: &Self
    ) -> Self {
        self.sub(&normal.mul_scalar(2.0 * self.dot(normal)))
    }
    
    pub fn angle_to(
        &self,
        other: &Self
    ) -> f32 {
        let d = (self.length_sq() * other.length_sq()).sqrt();
		if d == 0.0 {
            return PI / 2.0;
        };

		let theta = self.dot(other) / d;

		f32::acos(theta.clamp(-1.0, 1.0))
    }
    
    pub fn distance_to(
        &self,
        other: &Vector3
    ) -> f32 {
		f32::sqrt(self.distance_to_sq(other))
	}

	pub fn distance_to_sq(
        &self,
        other: &Vector3
    ) -> f32 {
		let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
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

		let w = 1.0 / (e[ 3] * x + e[ 7] * y + e[11] * z + e[15]);

		Self {
            x: (e[ 0] * x + e[ 4] * y + e[ 8] * z + e[12]) * w,
		    y: (e[ 1] * x + e[ 5] * y + e[ 9] * z + e[13]) * w,
		    z: (e[ 2] * x + e[ 6] * y + e[10] * z + e[14]) * w,
        }
    }

    pub fn apply_quaternion( 
        &self,
        q: &Quaternion
    ) -> Self {
		let tx = 2.0 * (q.y * self.z - q.z * self.y);
		let ty = 2.0 * (q.z * self.x - q.x * self.z);
		let tz = 2.0 * (q.x * self.y - q.y * self.x);

        Self {
            x: self.x + q.w * tx + q.y * tz - q.z * ty,
		    y: self.y + q.w * ty + q.z * tx - q.x * tz,
		    z: self.z + q.w * tz + q.x * ty - q.y * tx,
        }
    }

    pub fn rotate_x(
        &self,
        angle: f32
    ) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        
        Self {
            x: self.x,
            y: self.y * c - self.z * s,
            z: self.y * s + self.z * c,
        }
    }

    pub fn rotate_y(
        &self,
        angle: f32
    ) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        
        Self {
            x: self.x * c + self.z * s,
            y: self.y,
            z: -self.x * s + self.z * c,
        }
    }

    pub fn rotate_z(
        &self,
        angle: f32
    ) -> Self {
        let c = angle.cos();
        let s = angle.sin();

        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
            z: self.z,
        }
    }

    pub fn angle_zx(
        &self
    ) -> f32 {
        let a = self.z.atan2(self.x);
        if a < 0.0 { 
            a + 2.0 * PI
        } else { 
            a 
        }
    }

    pub fn angle_xz(
        &self
    ) -> f32 {
        let a = self.x.atan2(self.z);
        if a < 0.0 { 
            a + 2.0 * PI
        } else { 
            a 
        }
    }

}