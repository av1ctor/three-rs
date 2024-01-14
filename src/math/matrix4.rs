use super::{
    vector3::Vector3, 
    quaternion::Quaternion, 
    matrix3::Matrix3
};

const ZERO: Vector3 = Vector3{x: 0.0, y: 0.0, z: 0.0};
const ONE: Vector3 = Vector3{x: 1.0, y: 1.0, z: 1.0};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Matrix4(pub [f32; 4*4]);

impl Default for Matrix4 {
    fn default(
    ) -> Self {
        Self::identity()
    }
}

impl Matrix4 {
    pub fn from_slice(
        s: &[f32; 4*4]
    ) -> Self {
        Self(s.clone())
    }

    pub fn from_slice2(
        s: &[[f32; 4]]
    ) -> Self {
        Self([
            s[0][0], s[0][1], s[0][2], s[0][3],
            s[1][0], s[1][1], s[1][2], s[1][3],
            s[2][0], s[2][1], s[2][2], s[2][3],
            s[3][0], s[3][1], s[3][2], s[3][3],
        ])
    }

    pub fn from_translation(
        v: &Vector3
    ) -> Self {
        Self([
            0.0, 0.0, 0.0, v.x,
            0.0, 0.0, 0.0, v.y,
            0.0, 0.0, 0.0, v.z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn from_quaternion(
        q: &Quaternion
    ) -> Self {
        Self::compose(&ZERO, q, &ONE)
    }

    pub fn from_rotation_x( 
        theta: f32 
    ) -> Self {
		let c = theta.cos();
        let s = theta.sin();

		Self([
			1.0, 0.0, 0.0, 0.0,
			0.0,   c,  -s, 0.0,
			0.0,   s,   c, 0.0,
			0.0, 0.0, 0.0, 1.0
        ])
	}

	pub fn from_rotation_y( 
        theta: f32 
    ) -> Self {
		let c = theta.cos();
        let s = theta.sin();

		Self([
              c, 0.0,   s, 0.0,
            0.0, 1.0, 0.0, 0.0,
             -s, 0.0,   c, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
	}

	pub fn from_rotation_z( 
        theta: f32 
    ) -> Self {
		let c = theta.cos();
        let s = theta.sin();

		Self([
			  c,  -s, 0.0, 0.0,
			  s,   c, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
        ])
	}

    pub fn from_scale(
        x: f32, 
        y: f32, 
        z: f32 
    ) -> Self {

		Self([
              x, 0.0, 0.0, 0.0,
			0.0,   y, 0.0, 0.0,
			0.0, 0.0,   z, 0.0,
			0.0, 0.0, 0.0, 1.0
        ])
	}

	pub fn from_shear( 
        xy: f32, 
        xz: f32, 
        yx: f32, 
        yz: f32, 
        zx: f32, 
        zy: f32
    ) -> Self {

		Self([
			1.0,  yx,  zx, 0.0,
			 xy, 1.0,  zy, 0.0,
			 xz,  yz, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
        ])
	}

    pub fn identity(
    ) -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn to_slice(
        &self
    ) -> &[f32] {
        &self.0
    }

    pub fn mul(
        &self, 
        other: &Self
    ) -> Self {
        let a = &self.0;
		let b = &other.0;

		let a11 = a[ 0]; let a21 = a[ 1]; let a31 = a[ 2]; let a41 = a[ 3];
        let a12 = a[ 4]; let a22 = a[ 5]; let a32 = a[ 6]; let a42 = a[ 7];
        let a13 = a[ 8]; let a23 = a[ 9]; let a33 = a[10]; let a43 = a[11];
        let a14 = a[12]; let a24 = a[13]; let a34 = a[14]; let a44 = a[15];

		let b11 = b[ 0]; let b21 = b[ 1]; let b31 = b[ 2]; let b41 = b[ 3];
        let b12 = b[ 4]; let b22 = b[ 5]; let b32 = b[ 6]; let b42 = b[ 7];
        let b13 = b[ 8]; let b23 = b[ 9]; let b33 = b[10]; let b43 = b[11];
        let b14 = b[12]; let b24 = b[13]; let b34 = b[14]; let b44 = b[15];

		let e = [
            a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41,
            a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41,
            a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41,
            a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41,
            
            a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42,
            a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42,
            a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42,
            a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42,
            
            a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43,
            a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43,
            a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43,
            a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43,
            
            a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44,
            a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44,
            a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44,
            a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44
        ];

        Self(e)
    }

    pub fn mul_scalar(
        &self, 
        v: f32
    ) -> Self {
        let mut e = [0f32; 4*4];
        let s = &self.0;
        
        e[ 0] = s[ 0] * v; e[ 1] = s[ 1] * v; e[ 2] = s[ 2] * v; e[ 3] = s[ 3] * v;
        e[ 4] = s[ 4] * v; e[ 5] = s[ 5] * v; e[ 6] = s[ 6] * v; e[ 7] = s[ 7] * v;
        e[ 8] = s[ 8] * v; e[ 9] = s[ 9] * v; e[10] = s[10] * v; e[11] = s[11] * v;
        e[12] = s[12] * v; e[13] = s[13] * v; e[14] = s[14] * v; e[15] = s[15] * v;

        Self(e)
    }

    pub fn mul_vector(
        &self, 
        v: &Vector3
    ) -> Self {
        let mut e = [0f32; 4*4];
        let s = &self.0;
        
        e[ 0] = s[ 0] * v.x;
		e[ 1] = s[ 1] * v.y;
		e[ 2] = s[ 2] * v.z;
        e[ 3] = s[ 3];
		
        e[ 4] = s[ 4] * v.x;
		e[ 5] = s[ 5] * v.y;
		e[ 6] = s[ 6] * v.z;
        e[ 7] = s[ 7];

        e[ 8] = s[ 8] * v.x;
		e[ 9] = s[ 9] * v.y;
		e[10] = s[10] * v.z;
        e[11] = s[11];

        e[12] = s[12] * v.x;
		e[13] = s[13] * v.y;
		e[14] = s[14] * v.z;
        e[15] = s[15];

        Self(e)
    }

    pub fn determinant(
        &self
    ) -> f32 {
        let s = &self.0;

        let n11 = s[ 0]; let n21 = s[ 1]; let n31 = s[ 2]; let n41 = s[ 3]; 
        let n12 = s[ 4]; let n22 = s[ 5]; let n32 = s[ 6]; let n42 = s[ 7]; 
        let n13 = s[ 8]; let n23 = s[ 9]; let n33 = s[10]; let n43 = s[11]; 
        let n14 = s[12]; let n24 = s[13]; let n34 = s[14]; let n44 = s[15];

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

    pub fn invert(
        &self
    ) -> Self {
        let s = &self.0;

        let n11 = s[ 0]; let n21 = s[ 1]; let n31 = s[ 2]; let n41 = s[ 3]; 
        let n12 = s[ 4]; let n22 = s[ 5]; let n32 = s[ 6]; let n42 = s[ 7]; 
        let n13 = s[ 8]; let n23 = s[ 9]; let n33 = s[10]; let n43 = s[11]; 
        let n14 = s[12]; let n24 = s[13]; let n34 = s[14]; let n44 = s[15];

        let t11 = n23 * n34 * n42 - n24 * n33 * n42 + n24 * n32 * n43 - n22 * n34 * n43 - n23 * n32 * n44 + n22 * n33 * n44;
		let t12 = n14 * n33 * n42 - n13 * n34 * n42 - n14 * n32 * n43 + n12 * n34 * n43 + n13 * n32 * n44 - n12 * n33 * n44;
		let t13 = n13 * n24 * n42 - n14 * n23 * n42 + n14 * n22 * n43 - n12 * n24 * n43 - n13 * n22 * n44 + n12 * n23 * n44;
		let t14 = n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34 - n12 * n23 * n34;

		let det = n11 * t11 + n21 * t12 + n31 * t13 + n41 * t14;

		if det == 0.0 {
            return Self([
                0.0, 0.0, 0.0, 0.0, 
                0.0, 0.0, 0.0, 0.0, 
                0.0, 0.0, 0.0, 0.0, 
                0.0, 0.0, 0.0, 0.0
            ]);
        }

        let det_inv = 1.0 / det;

		Self([
            t11 * det_inv,
		    (n24 * n33 * n41 - n23 * n34 * n41 - n24 * n31 * n43 + n21 * n34 * n43 + n23 * n31 * n44 - n21 * n33 * n44) * det_inv,
		    (n22 * n34 * n41 - n24 * n32 * n41 + n24 * n31 * n42 - n21 * n34 * n42 - n22 * n31 * n44 + n21 * n32 * n44) * det_inv,
		    (n23 * n32 * n41 - n22 * n33 * n41 - n23 * n31 * n42 + n21 * n33 * n42 + n22 * n31 * n43 - n21 * n32 * n43) * det_inv,

		    t12 * det_inv,
		    (n13 * n34 * n41 - n14 * n33 * n41 + n14 * n31 * n43 - n11 * n34 * n43 - n13 * n31 * n44 + n11 * n33 * n44) * det_inv,
		    (n14 * n32 * n41 - n12 * n34 * n41 - n14 * n31 * n42 + n11 * n34 * n42 + n12 * n31 * n44 - n11 * n32 * n44) * det_inv,
		    (n12 * n33 * n41 - n13 * n32 * n41 + n13 * n31 * n42 - n11 * n33 * n42 - n12 * n31 * n43 + n11 * n32 * n43) * det_inv,

		    t13 * det_inv,
		    (n14 * n23 * n41 - n13 * n24 * n41 - n14 * n21 * n43 + n11 * n24 * n43 + n13 * n21 * n44 - n11 * n23 * n44) * det_inv,
		    (n12 * n24 * n41 - n14 * n22 * n41 + n14 * n21 * n42 - n11 * n24 * n42 - n12 * n21 * n44 + n11 * n22 * n44) * det_inv,
		    (n13 * n22 * n41 - n12 * n23 * n41 - n13 * n21 * n42 + n11 * n23 * n42 + n12 * n21 * n43 - n11 * n22 * n43) * det_inv,

		    t14 * det_inv,
		    (n13 * n24 * n31 - n14 * n23 * n31 + n14 * n21 * n33 - n11 * n24 * n33 - n13 * n21 * n34 + n11 * n23 * n34) * det_inv,
		    (n14 * n22 * n31 - n12 * n24 * n31 - n14 * n21 * n32 + n11 * n24 * n32 + n12 * n21 * n34 - n11 * n22 * n34) * det_inv,
		    (n12 * n23 * n31 - n13 * n22 * n31 + n13 * n21 * n32 - n11 * n23 * n32 - n12 * n21 * n33 + n11 * n22 * n33) * det_inv,
        ])
    }

    pub fn transpose(
        &self
    ) -> Self {
        let mut m = self.clone();
        let s = &self.0;

        // swap m21, m12
        m.0[ 1] = s[ 4];
        m.0[ 4] = s[ 1];
        // swap m31, m13
        m.0[ 2] = s[ 8];
        m.0[ 8] = s[ 2];
        // swap m32, m23
        m.0[ 6] = s[ 9];
        m.0[ 9] = s[ 6];

        // swap m41, m14
        m.0[ 3] = s[12];
        m.0[12] = s[ 3];
        // swap m42, m24
        m.0[ 7] = s[13];
        m.0[13] = s[ 7];
        // swap m43, m34
        m.0[11] = s[14];
        m.0[14] = s[11];

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

        let mut e = [0f32; 4*4];
        
        e[ 0] = (1.0 - (yy + zz)) * s.x;
		e[ 1] = (xy + wz) * s.x;
		e[ 2] = (xz - wy) * s.x;
		e[ 3] = 0.0;
		
        e[ 4] = (xy - wz) * s.y;
		e[ 5] = (1.0 - (xx + zz)) * s.y;
		e[ 6] = (yz + wx) * s.y;
		e[ 7] = 0.0;
		
        e[ 8] = (xz + wy) * s.z;
		e[ 9] = (yz - wx) * s.z;
		e[10] = (1.0 - ( xx + yy )) * s.z;
		e[11] = 0.0;

		e[12] = pos.x;
		e[13] = pos.y;
		e[14] = pos.z;
		e[15] = 1.0;

        Self(e)
    }

    pub fn decompose(
        &self
    ) -> (Vector3, Quaternion, Vector3) {
        let e = &self.0;

        let mut sx = Vector3::new(e[ 0], e[ 1], e[ 2]).length();
		let sy = Vector3::new(e[ 4], e[ 5], e[ 6]).length();
		let sz = Vector3::new(e[ 8], e[ 9], e[10]).length();

		let det = self.determinant();
		if det < 0.0 {
            sx = -sx
        };

		let position = Vector3::new(e[12], e[13], e[14]);

		let mut rm = Matrix3::from_matrix4(&self);

		let inv_sx = 1.0 / sx;
		let inv_sy = 1.0 / sy;
		let inv_sz = 1.0 / sz;

		rm.0[ 0] *= inv_sx;
		rm.0[ 1] *= inv_sx;
		rm.0[ 2] *= inv_sx;

		rm.0[ 4] *= inv_sy;
		rm.0[ 5] *= inv_sy;
		rm.0[ 6] *= inv_sy;

		rm.0[ 8] *= inv_sz;
		rm.0[ 9] *= inv_sz;
		rm.0[10] *= inv_sz;

		let quaternion = Quaternion::from_matrix(&rm);

		let scale = Vector3::new(sx, sy, sz);

        (position, quaternion, scale)
    }

    pub fn look_at( 
        eye: &Vector3, 
        target: &Vector3, 
        up: &Vector3
    ) -> Self {
		let mut z = eye.sub(target);
		if  z.length_sq() == 0.0 {
			z.z = 1.0;
		}

		z = z.normalize();
        let mut x = up.cross(&z);

		if x.length_sq() == 0.0 {
			if up.z.abs() == 1.0 {
				z.x += 0.0001;
			} else {
				z.z += 0.0001;
			}

			z = z.normalize();
			x = up.cross(&z);
		}

		x = x.normalize();
		let y = z.cross(&x);

		Self([
            x.x, x.y, x.z, 0.0,
            y.x, y.y, y.z, 0.0,
            z.x, z.y, z.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
	}

    pub fn perspective(
        left: f32, 
        right: f32, 
        top: f32, 
        bottom: f32, 
        near: f32, 
        far: f32
    ) -> Self {
		let x = 2.0 * near / (right - left);
		let y = 2.0 * near / (top - bottom);

		let a = (right + left) / (right - left);
		let b = (top + bottom) / (top - bottom);

		let c = -(far + near) / (far - near);
        let d = (-2.0 * far * near) / (far - near);

		Self([
              x, 0.0, 0.0,  0.0,	
            0.0,   y, 0.0,  0.0,	
              a,   b,   c, -1.0,	
            0.0, 0.0,   d,  0.0,
        ])
	}

    pub fn perspective_fov(
        fov: f32,
        aspect: f32,
        near: f32, 
        far: f32
    ) -> Self {
		let tan_half_fov = (fov / 2.0).tan();

        let x = 1.0 / (aspect * tan_half_fov);
        let y = 1.0 / tan_half_fov;

        let a = -(far + near) / (far - near);
        let b = -(2.0 * far * near) / (far - near);
        
        Self([
             x, 0.0, 0.0,  0.0,	
            0.0,  y, 0.0,  0.0,	
            0.0, 0.0,  a, -1.0,	
            0.0, 0.0,  b,  0.0,
        ])
    }

    pub fn rotate(
        &self,
        angle: f32,
        axis: &Vector3
    ) -> Self {
        let m = &self.0;
        let a = angle;
		let c = a.cos();
		let s = a.sin();

		let axis = axis.normalize();
		let temp = axis.mul_scalar(1.0 - c);

		let mut rot = [0.0f32; 4*4];
		rot[ 0] = c + temp.x * axis.x;
		rot[ 1] = temp.x * axis.y + s * axis.z;
		rot[ 2] = temp.x * axis.z - s * axis.y;

		rot[ 4] = temp.y * axis.x - s * axis.z;
		rot[ 5] = c + temp.y * axis.y;
		rot[ 6] = temp.y * axis.z + s * axis.x;

		rot[ 8] = temp.z * axis.x + s * axis.y;
		rot[ 9] = temp.z * axis.y - s * axis.x;
		rot[10] = c + temp.z * axis.z;

		let mut res = [0.0f32; 4*4];
		res[0] = m[0] * rot[ 0] + m[1] * rot[ 1] + m[2] * rot[ 2];
		res[1] = m[0] * rot[ 4] + m[1] * rot[ 5] + m[2] * rot[ 6];
		res[2] = m[0] * rot[ 8] + m[1] * rot[ 9] + m[2] * rot[10];
		res[3] = m[3];

        Self(res)
    }
}