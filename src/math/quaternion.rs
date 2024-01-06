use serde::{Serialize, Deserialize};
use super::{
    euler::{EulerOrder, Euler}, 
    vector3::{Vector3, RIGHT, UP, FORWARD}, 
    matrix3::Matrix3
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Quaternion {
    fn default(
    ) -> Self {
        Self::identity()
    }
}

impl Quaternion {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        w: f32
    ) -> Self {
        Self {
            x,
            y,
            z,
            w,
        }
    }

    pub fn from_slice(
        src: &[f32; 4]
    ) -> Self {
        Self {
            x: src[0],
            y: src[1],
            z: src[2],
            w: src[3],
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
            w: src[i+3],
        }
    }

    pub fn from_vector(
        vector: &Vector3,
        order: EulerOrder
    ) -> Self {
		let c1 = (vector.x / 2.0).cos();
		let c2 = (vector.y / 2.0).cos();
		let c3 = (vector.z / 2.0).cos();

		let s1 = (vector.x / 2.0).sin();
		let s2 = (vector.y / 2.0).sin();
		let s3 = (vector.z / 2.0).sin();

		match order {
			EulerOrder::XYZ => {
				Self {
                    x: s1 * c2 * c3 + c1 * s2 * s3,
                    y: c1 * s2 * c3 - s1 * c2 * s3,
                    z: c1 * c2 * s3 + s1 * s2 * c3,
                    w: c1 * c2 * c3 - s1 * s2 * s3,
                }
            }
        }
    }

    pub fn from_euler(
        euler: &Euler
    ) -> Self {
        Self::from_vector(&euler.v, euler.order)
    }

    pub fn from_matrix(
        m: &Matrix3
    ) -> Self {
        let m11 = m.0[0][0];
        let m21 = m.0[0][1];
        let m31 = m.0[0][2];
        
        let m12 = m.0[1][0];
        let m22 = m.0[1][1];
        let m32 = m.0[1][2];

        let m13 = m.0[2][0];
        let m23 = m.0[2][1];
        let m33 = m.0[2][2];

        let trace = m11 + m22 + m33;

        if trace > 0.0 {
			let s = 0.5 / (trace + 1.0).sqrt();

			Self {
                x: (m32 - m23) * s,
                y: (m13 - m31) * s,
                z: (m21 - m12) * s,
                w: 0.25 / s,
            }
		}
        else if m11 > m22 && m11 > m33 {
			let s = 2.0 * (1.0 + m11 - m22 - m33).sqrt();

			Self {
                x: 0.25 * s,
                y: (m12 + m21) / s,
                z: (m13 + m31) / s,
                w: (m32 - m23) / s,
            }
		} 
        else if m22 > m33 {
			let s = 2.0 * (1.0 + m22 - m11 - m33).sqrt();

			Self {
                x: (m12 + m21) / s,
                y: 0.25 * s,
                z: (m23 + m32) / s,
                w: (m13 - m31) / s,
            }
		} 
        else {
			let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();

			Self {
                x: (m13 + m31) / s,
                y: (m23 + m32) / s,
                z: 0.25 * s,
                w: (m21 - m12) / s,
            }
		}
    }

    pub fn from_axis_and_angle(
        axis: &Vector3,
        angle: f32
    ) -> Self {
        let half_angle = angle / 2.0; 
        let s = half_angle.sin();

        Self {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: half_angle.cos(),
        }
    }

    pub fn to_slice(
        &self
    ) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn identity(
    ) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn mul(
        &self,
        other: &Self
    ) -> Self {
        let ax = self.x; 
        let ay = self.y;
        let az = self.z;
        let aw = self.w;
		let bx = other.x;
        let by = other.y;
        let bz = other.z;
        let bw = other.w;

        Self {
            x: ax * bw + aw * bx + ay * bz - az * by,
            y: ay * bw + aw * by + az * bx - ax * bz,
            z: az * bw + aw * bz + ax * by - ay * bx,
            w: aw * bw - ax * bx - ay * by - az * bz,
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
            w: self.w * scalar,
        }
    }

    pub fn dot( 
        &self,
        other: &Self
    ) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w
	}

    pub fn length_sq(
        &self
    ) -> f32 {
		self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
	}

    pub fn length(
        &self
    ) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
	}

    pub fn normalize(
        &self
    ) -> Self {
		let len = self.length();
        if len == 0.0 {
            Self::identity()
        }
        else {
            self.mul_scalar(1.0 / len)
        }
	}

    pub fn inverse(
        &self
    ) -> Self {
        Self {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
            w: self.w,
        }
    }

    pub fn rotate_on_axis(
        &self,
        axis: &Vector3,
        angle: f32
    ) -> Self {
        self.mul(&Quaternion::from_axis_and_angle(axis, angle))
    }

    pub fn rotate_x(
        &self,
        angle: f32
    ) -> Self {
        self.rotate_on_axis(&RIGHT, angle)
    }

    pub fn rotate_y(
        &self,
        angle: f32
    ) -> Self {
        self.rotate_on_axis(&UP, angle)
    }

    pub fn rotate_z(
        &self,
        angle: f32
    ) -> Self {
        self.rotate_on_axis(&FORWARD, angle)
    }

    pub fn angle(
        &self
    ) -> f32 {
        2.0 * self.w.acos()
    }

    pub fn yaw(
        &self
    ) -> f32 {
        f32::atan2(
            2.0 * (self.y * self.z + self.w * self.x), 
            self.w * self.w - self.x * self.x - self.y * self.y + self.z * self.z
        )
    }

    pub fn pitch(
        &self
    ) -> f32 {
        f32::asin(-2.0 * (self.x * self.z - self.w * self.y))
    }

    pub fn roll(
        &self
    ) -> f32 {
        f32::atan2(
            2.0 * (self.x * self.y + self.w * self.z), 
            self.w * self.w + self.x * self.x - self.y * self.y - self.z * self.z
        )
    }
}