
#[derive(Clone, Debug)]
pub struct Matrix4(pub [[f32; 4]; 4]);

impl Matrix4 {
    pub fn new(
        src: &[[f32; 4]; 4]
    ) -> Self {
        Self(src.clone())
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
}