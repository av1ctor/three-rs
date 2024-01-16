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

		Self([
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
        ])
    }

    pub fn mul_scalar(
        &self, 
        v: f32
    ) -> Self {
        let s = &self.0;
        
        Self([
            s[ 0] * v, 
            s[ 1] * v, 
            s[ 2] * v, 
            s[ 3] * v,
            
            s[ 4] * v, 
            s[ 5] * v, 
            s[ 6] * v, 
            s[ 7] * v,
            
            s[ 8] * v, 
            s[ 9] * v, 
            s[10] * v, 
            s[11] * v,
            
            s[12] * v, 
            s[13] * v, 
            s[14] * v, 
            s[15] * v
        ])
    }

    pub fn mul_vector(
        &self, 
        v: &Vector3
    ) -> Self {
        let s = &self.0;
        
        Self([
            s[ 0] * v.x,
            s[ 1] * v.y,
            s[ 2] * v.z,
            s[ 3],
            
            s[ 4] * v.x,
            s[ 5] * v.y,
            s[ 6] * v.z,
            s[ 7],
    
            s[ 8] * v.x,
            s[ 9] * v.y,
            s[10] * v.z,
            s[11],
    
            s[12] * v.x,
            s[13] * v.y,
            s[14] * v.z,
            s[15]
        ])
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

        Self([
            (1.0 - (yy + zz)) * s.x,
            (xy + wz) * s.x,
            (xz - wy) * s.x,
            0.0,
            
            (xy - wz) * s.y,
            (1.0 - (xx + zz)) * s.y,
            (yz + wx) * s.y,
            0.0,
            
            (xz + wy) * s.z,
            (yz - wx) * s.z,
            (1.0 - ( xx + yy )) * s.z,
            0.0,
    
            pos.x,
            pos.y,
            pos.z,
            1.0
        ])
    }

    pub fn decompose(
        &self
    ) -> (Vector3, Quaternion, Vector3) {
        let e = &self.0;

        let mut sx = Vector3::new(e[ 0], e[ 1], e[ 2]).length();
		let     sy = Vector3::new(e[ 4], e[ 5], e[ 6]).length();
		let     sz = Vector3::new(e[ 8], e[ 9], e[10]).length();

		let det = self.determinant();
		if det < 0.0 {
            sx = -sx
        };

		let mut rm = Matrix3::from_matrix4(&self);

		let inv_sx = 1.0 / sx;
		let inv_sy = 1.0 / sy;
		let inv_sz = 1.0 / sz;

		rm.0[0] *= inv_sx;
		rm.0[1] *= inv_sx;
		rm.0[2] *= inv_sx;

		rm.0[3] *= inv_sy;
		rm.0[4] *= inv_sy;
		rm.0[5] *= inv_sy;

		rm.0[6] *= inv_sz;
		rm.0[7] *= inv_sz;
		rm.0[8] *= inv_sz;

		let position = Vector3::new(e[12], e[13], e[14]);
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

    pub fn orthographic(
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32
    ) -> Self {
        let w = 1.0 / (right - left);
		let h = 1.0 / (top - bottom);
		let p = 1.0 / (far - near);

		let x = (right + left) * w;
		let y = (top + bottom) * h;

		let z = ( far + near ) * p;
        let z_inv = -2.0 * p;

        Self([
            2.0 * w, 0.0    ,   0.0, 0.0, 		
            0.0    , 2.0 * h,   0.0, 0.0,		
            0.0    , 0.0    , z_inv, 0.0,		
                 -x,      -y,    -z, 1.0
        ])
    }

    pub fn rotate(
        &self,
        a: f32,
        axis: &Vector3
    ) -> Self {
        let m = &self.0;
		let c = a.cos();
		let s = a.sin();

		let axis = axis.normalize();
		let temp = axis.mul_scalar(1.0 - c);

		let rot = [
            c + temp.x * axis.x,
            temp.x * axis.y + s * axis.z,
            temp.x * axis.z - s * axis.y,

            temp.y * axis.x - s * axis.z,
            c + temp.y * axis.y,
            temp.y * axis.z + s * axis.x,

            temp.z * axis.x + s * axis.y,
            temp.z * axis.y - s * axis.x,
            c + temp.z * axis.z
        ];

		Self([
            m[0] * rot[0] + m[1] * rot[1] + m[2] * rot[2],
            m[0] * rot[3] + m[1] * rot[4] + m[2] * rot[5],
            m[0] * rot[6] + m[1] * rot[7] + m[2] * rot[8],
            m[3],
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        ])
    }
}