use serde::{Serialize, Deserialize};
use super::{
    {EulerOrder, Euler}, 
    {Vector3, RIGHT, UP, FORWARD}, 
    Matrix3
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

    pub fn from_euler(
        euler: &Euler
    ) -> Self {
		let vector = &euler.v;

        let c1 = (vector.x / 2.0).cos();
		let c2 = (vector.y / 2.0).cos();
		let c3 = (vector.z / 2.0).cos();

		let s1 = (vector.x / 2.0).sin();
		let s2 = (vector.y / 2.0).sin();
		let s3 = (vector.z / 2.0).sin();

		match euler.order {
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

    pub fn from_matrix(
        m: &Matrix3
    ) -> Self {
        let m11 = m.0[0]; let m21 = m.0[1]; let m31 = m.0[2];
        let m12 = m.0[3]; let m22 = m.0[4]; let m32 = m.0[5];
        let m13 = m.0[6]; let m23 = m.0[7]; let m33 = m.0[8];

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

    pub fn from_unit_vector(
        from: &Vector3,
        to: &Vector3
    ) -> Self {
		let r = from.dot(to) + 1.0;

		if r < f32::EPSILON {
			if from.x.abs() > from.z.abs() {
				Self {
                    x: -from.y,
				    y: from.x,
				    z: 0.0,
				    w: 0.0
                }
			} 
            else {
				Self {
                    x: 0.0,
				    y: -from.z,
				    z: from.y,
				    w: 0.0
                }
			}
		} 
        else {
        	let c = from.cross(&to);
            Self {
                x: c.x,
                y: c.y,
                z: c.y,
                w: r
            }
		}.normalize()
	}

    pub fn from_look_rotation(
        forward: &Vector3,
        up: &Vector3
    ) -> Self {
        let forward = forward.normalize();
        let right = up.cross(&forward).normalize();
        let up = forward.cross(&right);
        let m = Matrix3::new([
            right.x, right.y, right.z,
            up.x, up.y, up.z,
            forward.x, forward.y, forward.z
        ]);

        Self::from_matrix(&m)
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

    pub fn angle_to(
        &self,
        q: &Self
    ) -> f32 {
		2.0 * self.dot(q).clamp(-1.0, 1.0).abs().acos()
	}

	pub fn rotate_towards(
        &self,
        q: &Self, 
        step: f32 
    ) -> Self {
		let angle = self.angle_to(q);

		if angle == 0.0 {
            self.clone()
        }
        else {
            let t = f32::min(1.0, step / angle);
            self.slerp(q, t)
        }
	}

    pub fn slerp(
        &self,
        other: &Self,
        t: f32
    ) -> Self {
        if t == 0.0 {
            self.clone()
        }
        else if t == 1.0 {
            other.clone()
        }
        else {
            let x = self.x;
            let y = self.y; 
            let z = self.z; 
            let w = self.w;

            let mut cos_half_theta = 
                w * other.w + 
                x * other.x + 
                y * other.y + 
                z * other.z;
                
            let q = if cos_half_theta < 0.0 {
                cos_half_theta = -cos_half_theta;

                Self{
                    w: -other.w,
                    x: -other.x,
                    y: -other.y,
                    z: -other.z,
                }
            } 
            else {
                other.clone()
            };

            if cos_half_theta >= 1.0 {
                return Self {
                    w,
                    x,
                    y,
                    z,
                };
            }

            let sqr_sin_half_theta = 
                1.0 - cos_half_theta * cos_half_theta;
            if sqr_sin_half_theta <= f32::EPSILON {
                let s = 1.0 - t;
                
                return Self {
                    w: s * w + t * q.w,
                    x: s * x + t * q.x,
                    y: s * y + t * q.y,
                    z: s * z + t * q.z,
                }.normalize();
            }

            let sin_half_theta = sqr_sin_half_theta.sqrt();
            let half_theta = f32::atan2(sin_half_theta, cos_half_theta);
            let ratio_a = ((1.0 - t) * half_theta).sin() / sin_half_theta;
            let ratio_b = (t * half_theta).sin() / sin_half_theta;

            Self {
                w: w * ratio_a + q.w * ratio_b,
                x: x * ratio_a + q.x * ratio_b,
                y: y * ratio_a + q.y * ratio_b,
                z: z * ratio_a + q.z * ratio_b,
            }
        }
    }

    pub fn average(
        quaternions: &Vec<Self>
    ) -> Self {
        let mut forward_acc = Vector3::zero();
        let mut up_acc = Vector3::zero();
        
        for q in quaternions {
            forward_acc.add_mut(&FORWARD.apply_quaternion(q));
            up_acc.add_mut(&UP.apply_quaternion(q));
        }

        forward_acc = forward_acc.div_scalar(quaternions.len() as f32);
        up_acc = up_acc.div_scalar(quaternions.len() as f32);

        Self::from_look_rotation(&forward_acc, &up_acc)
    }

    pub fn average_weighted(
        quaternions_weights: &Vec<(Self, f32)>
    ) -> Self {
        let mut forward_acc = Vector3::zero();
        let mut up_acc = Vector3::zero();
        
        for qw in quaternions_weights {
            forward_acc.add_mut(&FORWARD.apply_quaternion(&qw.0).mul_scalar(qw.1));
            up_acc.add_mut(&UP.apply_quaternion(&qw.0).mul_scalar(qw.1));
        }

        forward_acc = forward_acc.div_scalar(quaternions_weights.len() as f32);
        up_acc = up_acc.div_scalar(quaternions_weights.len() as f32);

        Self::from_look_rotation(&forward_acc, &up_acc)
    }
}