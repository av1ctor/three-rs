pub use serde::{Serialize, Deserialize};

use super::{vector3::Vector3, matrix4::Matrix4, quaternion::Quaternion};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum EulerOrder {
    XYZ
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Euler {
    pub v: Vector3,
    pub order: EulerOrder,
}

impl Default for Euler {
    fn default(
    ) -> Self {
        Self {
            v: Vector3::zero(),
            order: EulerOrder::XYZ
        }
    }
}

impl Euler {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        order: EulerOrder
    ) -> Self {
        Self {
            v: Vector3::new(x, y, z),
            order
        }
    }

    pub fn from_vector(
        v: Vector3,
        order: EulerOrder
    ) -> Self {
        Self {
            v,
            order
        }
    }

    pub fn from_quartenion(
        q: &Quaternion,
        order: EulerOrder
    ) -> Self {
        let m = Matrix4::from_quartenion(q);
        Self::from_rotation_matrix(&m, order)
    }

    pub fn from_rotation_matrix(
        m: &Matrix4,
        order: EulerOrder
    ) -> Self {
        let m11 = m.0[0][0];
        let m12 = m.0[1][0];
        let m13 = m.0[2][0];
        let m22 = m.0[1][1];
        let m23 = m.0[2][1];
        let m32 = m.0[1][2];
        let m33 = m.0[3][2];

		match order {
            EulerOrder::XYZ => {
                let y = f32::asin(m13.clamp(-1.0, 1.0));
                let (x, z) = if m13.abs() < 0.9999999  {
					(
                        f32::atan2(-m23, m33),
					    f32::atan2(-m12, m11)
                    )
				} else {
                    (
                        f32::atan2(m32, m22),
					    0.0
                    )
				};

                Self {
                    v: Vector3::new(x, y, z),
                    order,
                }
            }
        }
    }
}