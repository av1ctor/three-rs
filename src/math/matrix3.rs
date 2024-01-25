use super::{vector3::Vector3, matrix4::Matrix4};

#[derive(Clone, Debug)]
pub struct Matrix3(pub [f32; 3*3]);

impl Default for Matrix3 {
    fn default(
    ) -> Self {
        Self::identity()
    }
}


impl Matrix3 {
    pub fn new(
        src: [f32; 3*3]
    ) -> Self {
        Self(src)
    }

    pub fn from_matrix4(
        m: &Matrix4
    ) -> Self {
        let mut e = [0f32; 3*3];

        e[0] = m.0[0]; e[1] = m.0[1]; e[2] = m.0[2];
        e[3] = m.0[4]; e[4] = m.0[5]; e[5] = m.0[6];
        e[6] = m.0[8]; e[7] = m.0[9]; e[8] = m.0[10];

        Self(e)
    }

    pub fn identity(
    ) -> Self {
        Self([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ])
    }

    pub fn mul_scalar(
        &self, 
        scalar: f32
    ) -> Self {
        let mut e = [0f32; 3*3];
        let s = &self.0;
        
        e[0] = s[0] * scalar; e[1] = s[1] * scalar; e[2] = s[2] * scalar;
        e[3] = s[3] * scalar; e[4] = s[4] * scalar; e[5] = s[5] * scalar;
        e[6] = s[6] * scalar; e[7] = s[7] * scalar; e[8] = s[8] * scalar;

        Self(e)
    }

    pub fn mul_vector(
        &self, 
        v: &Vector3
    ) -> Self {
        let mut e = [0f32; 3*3];
        let s = &self.0;
        
        e[0] = s[0] * v.x; e[1] = s[1] * v.y; e[2] = s[2] * v.z;
        e[3] = s[3] * v.x; e[4] = s[4] * v.y; e[5] = s[5] * v.z;
        e[6] = s[6] * v.x; e[7] = s[7] * v.y; e[8] = s[8] * v.z;

        Self(e)
    }
}