use super::{vector3::Vector3, quaternion::Quaternion, matrix3::Matrix3};

const ZERO: Vector3 = Vector3{x: 0.0, y: 0.0, z: 0.0};
const ONE: Vector3 = Vector3{x: 1.0, y: 1.0, z: 1.0};

#[derive(Clone, Debug, Default)]
pub struct Matrix4(pub [[f32; 4]; 4]);

impl Matrix4 {
    pub fn new(
        src: &[[f32; 4]; 4]
    ) -> Self {
        Self(src.clone())
    }

    pub fn from_quartenion(
        q: &Quaternion
    ) -> Self {
        Self::compose(&ZERO, q, &ONE)
    }

    pub fn mul(
        &self, 
        other: &Self
    ) -> Self {
        let a = &self.0;
		let b = &other.0;

		let a11 = a[0][0];
        let a21 = a[0][1];
        let a31 = a[0][2];
        let a41 = a[0][3];

        let a12 = a[1][0];
        let a22 = a[1][1];
        let a32 = a[1][2];
        let a42 = a[1][3];

        let a13 = a[2][0];
        let a23 = a[2][1];
        let a33 = a[2][2];
        let a43 = a[2][3];

        let a14 = a[3][0];
        let a24 = a[3][1];
        let a34 = a[3][2];
        let a44 = a[3][3];

		let b11 = b[0][0];
        let b21 = b[0][1];
        let b31 = b[0][2];
        let b41 = b[0][3];
        
        let b12 = b[1][0];
        let b22 = b[1][1];
        let b32 = b[1][2];
        let b42 = b[1][3];
        
        let b13 = b[2][0];
        let b23 = b[2][1];
        let b33 = b[2][2];
        let b43 = b[2][3];
        
        let b14 = b[3][0];
        let b24 = b[3][1];
        let b34 = b[3][2];
        let b44 = b[3][3];

		let mut e = [[0f32; 4]; 4];
        
        e[0][0] = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
		e[0][1] = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
		e[0][2] = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
		e[0][3] = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
		
        e[1][0] = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
		e[1][1] = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
		e[1][2] = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
		e[1][3] = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
		
        e[2][0] = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
		e[2][1] = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
		e[2][2] = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
		e[2][3] = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
		
        e[3][0] = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;
		e[3][1] = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;
		e[3][2] = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;
		e[3][3] = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;

        Self(e)
    }

    pub fn mul_scalar(
        &self, 
        scalar: f32
    ) -> Self {
        let mut e = [[0f32; 4]; 4];
        
        e[0][0] = self.0[0][0] * scalar;
		e[0][1] = self.0[0][1] * scalar;
		e[0][2] = self.0[0][2] * scalar;
		e[0][3] = self.0[0][3] * scalar;
		
        e[1][0] = self.0[1][0] * scalar;
		e[1][1] = self.0[1][1] * scalar;
		e[1][2] = self.0[1][2] * scalar;
		e[1][3] = self.0[1][3] * scalar;

        e[2][0] = self.0[2][0] * scalar;
		e[2][1] = self.0[2][1] * scalar;
		e[2][2] = self.0[2][2] * scalar;
		e[2][3] = self.0[2][3] * scalar;

        e[3][0] = self.0[3][0] * scalar;
		e[3][1] = self.0[3][1] * scalar;
		e[3][2] = self.0[3][2] * scalar;
		e[3][3] = self.0[3][3] * scalar;

        Self(e)
    }

    pub fn mul_vector(
        &self, 
        v: &Vector3
    ) -> Self {
        let mut e = [[0f32; 4]; 4];
        
        e[0][0] = self.0[0][0] * v.x;
		e[0][1] = self.0[0][1] * v.y;
		e[0][2] = self.0[0][2] * v.z;
        e[0][3] = self.0[0][3];
		
        e[1][0] = self.0[1][0] * v.x;
		e[1][1] = self.0[1][1] * v.y;
		e[1][2] = self.0[1][2] * v.z;
        e[1][3] = self.0[1][3];

        e[2][0] = self.0[2][0] * v.x;
		e[2][1] = self.0[2][1] * v.y;
		e[2][2] = self.0[2][2] * v.z;
        e[2][3] = self.0[2][3];

        e[3][0] = self.0[3][0] * v.x;
		e[3][1] = self.0[3][1] * v.y;
		e[3][2] = self.0[3][2] * v.z;
        e[3][3] = self.0[3][3];

        Self(e)
    }

    pub fn determinant(
        &self
    ) -> f32 {
        let n11 = self.0[0][0]; 
        let n21 = self.0[0][1]; 
        let n31 = self.0[0][2]; 
        let n41 = self.0[0][3]; 

        let n12 = self.0[1][0]; 
        let n22 = self.0[1][1]; 
        let n32 = self.0[1][2]; 
        let n42 = self.0[1][3]; 

        let n13 = self.0[2][0]; 
        let n23 = self.0[2][1]; 
        let n33 = self.0[2][2]; 
        let n43 = self.0[2][3]; 

        let n14 = self.0[3][0];
        let n24 = self.0[3][1];
        let n34 = self.0[3][2];
        let n44 = self.0[3][3];

        n41 * (
            n14 * n23 * n32
            - n13 * n24 * n32
            - n14 * n22 * n33
            + n12 * n24 * n33
            + n13 * n22 * n34
            - n12 * n23 * n34
        ) +
        n42 * (
            n11 * n23 * n34
            - n11 * n24 * n33
            + n14 * n21 * n33
            - n13 * n21 * n34
            + n13 * n24 * n31
            - n14 * n23 * n31
        ) +
        n43 * (
            n11 * n24 * n32
            - n11 * n22 * n34
            - n14 * n21 * n32
            + n12 * n21 * n34
            + n14 * n22 * n31
            - n12 * n24 * n31
        ) +
        n44 * (
            - n13 * n22 * n31
            - n11 * n23 * n32
            + n11 * n22 * n33
            + n13 * n21 * n32
            - n12 * n21 * n33
            + n12 * n23 * n31
        )
    }

    pub fn transpose(
        &self
    ) -> Self {
        let mut m = self.clone();

        // swap m21, m12
        m.0[0][1] = self.0[1][0];
        m.0[1][0] = self.0[0][1];
        // swap m31, m13
        m.0[0][2] = self.0[2][0];
        m.0[2][0] = self.0[0][2];
        // swap m32, m23
        m.0[1][2] = self.0[2][1];
        m.0[2][1] = self.0[1][2];

        // swap m41, m14
        m.0[0][3] = self.0[3][0];
        m.0[3][0] = self.0[0][3];
        // swap m42, m24
        m.0[1][3] = self.0[3][1];
        m.0[3][1] = self.0[1][3];
        // swap m43, m34
        m.0[2][3] = self.0[3][2];
        m.0[3][2] = self.0[2][3];

        m
    }

    pub fn compose(
        pos: &Vector3,
        q: &Quaternion,
        s: &Vector3
    ) -> Self {
		let x = q.x;
        let y = q.y;
        let z = q.z;
        let w = q.w;
		
        let x2 = x + x;	
        let y2 = y + y;
        let z2 = z + z;
		
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
		
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
		
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let mut e = [[0f32; 4]; 4];
        
        e[0][0] = (1.0 - (yy + zz)) * s.x;
		e[0][1] = (xy + wz) * s.x;
		e[0][2] = (xz - wy) * s.x;
		e[0][3] = 0.0;
		
        e[1][0] = (xy - wz) * s.y;
		e[1][1] = (1.0 - (xx + zz)) * s.y;
		e[1][2] = (yz + wx) * s.y;
		e[1][3] = 0.0;
		
        e[2][0] = (xz + wy) * s.z;
		e[2][1] = (yz - wx) * s.z;
		e[2][2] = (1.0 - ( xx + yy )) * s.z;
		e[2][3] = 0.0;

		e[3][0] = pos.x;
		e[3][1] = pos.y;
		e[3][2] = pos.z;
		e[3][3] = 1.0;

        Self(e)
    }

    pub fn decompose(
        &self
    ) -> (Vector3, Quaternion, Vector3) {
        let e = &self.0;

        let mut sx = Vector3::new(e[0][0], e[0][1], e[0][2]).length();
		let sy = Vector3::new(e[1][0], e[1][1], e[1][2]).length();
		let sz = Vector3::new(e[2][0], e[2][1], e[2][2]).length();

		let det = self.determinant();
		if det < 0.0 {
            sx = -sx
        };

		let position = Vector3::new(e[3][0], e[3][1], e[3][2]);

		let mut rm = Matrix3::from_matrix4(&self);

		let inv_sx = 1.0 / sx;
		let inv_sy = 1.0 / sy;
		let inv_sz = 1.0 / sz;

		rm.0[0][0] *= inv_sx;
		rm.0[0][1] *= inv_sx;
		rm.0[0][2] *= inv_sx;

		rm.0[1][0] *= inv_sy;
		rm.0[1][1] *= inv_sy;
		rm.0[1][2] *= inv_sy;

		rm.0[2][0] *= inv_sz;
		rm.0[2][1] *= inv_sz;
		rm.0[2][2] *= inv_sz;

		let quaternion = Quaternion::from_matrix(&rm);

		let scale = Vector3::new(sx, sy, sz);

        (position, quaternion, scale)
    }
}