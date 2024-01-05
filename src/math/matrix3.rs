use super::{vector3::Vector3, matrix4::Matrix4};

#[derive(Clone, Debug, Default)]
pub struct Matrix3(pub [[f32; 3]; 3]);

impl Matrix3 {
    pub fn new(
        src: &[[f32; 3]; 3]
    ) -> Self {
        Self(src.clone())
    }

    pub fn from_matrix4(
        m: &Matrix4
    ) -> Self {
        let mut e = [[0f32; 3]; 3];

        e[0][0] = m.0[0][0];
		e[0][1] = m.0[0][1];
		e[0][2] = m.0[0][2];
		
        e[1][0] = m.0[1][0];
		e[1][1] = m.0[1][1];
		e[1][2] = m.0[1][2];

        e[2][0] = m.0[2][0];
		e[2][1] = m.0[2][1];
		e[2][2] = m.0[2][2];

        Self(e)
    }

    pub fn mul_scalar(
        &self, 
        scalar: f32
    ) -> Self {
        let mut e = [[0f32; 3]; 3];
        
        e[0][0] = self.0[0][0] * scalar;
		e[0][1] = self.0[0][1] * scalar;
		e[0][2] = self.0[0][2] * scalar;
		
        e[1][0] = self.0[1][0] * scalar;
		e[1][1] = self.0[1][1] * scalar;
		e[1][2] = self.0[1][2] * scalar;

        e[2][0] = self.0[2][0] * scalar;
		e[2][1] = self.0[2][1] * scalar;
		e[2][2] = self.0[2][2] * scalar;

        Self(e)
    }

    pub fn mul_vector(
        &self, 
        v: &Vector3
    ) -> Self {
        let mut e = [[0f32; 3]; 3];
        
        e[0][0] = self.0[0][0] * v.x;
		e[0][1] = self.0[0][1] * v.y;
		e[0][2] = self.0[0][2] * v.z;
		
        e[1][0] = self.0[1][0] * v.x;
		e[1][1] = self.0[1][1] * v.y;
		e[1][2] = self.0[1][2] * v.z;

        e[2][0] = self.0[2][0] * v.x;
		e[2][1] = self.0[2][1] * v.y;
		e[2][2] = self.0[2][2] * v.z;

        Self(e)
    }
}