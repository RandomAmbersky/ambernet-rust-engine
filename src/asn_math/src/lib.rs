pub const IDENTITY_MATRIX: [f32;16] = [
1., 0., 0., 0.,
0., 1., 0., 0.,
0., 0., 1., 0.,
0., 0., 0., 1.
];

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
	let mut return_var = [0.; 16];

	return_var[0] = 1.;
	return_var[5] = 1.;
	return_var[10] = 1.;
	return_var[15] = 1.;

	return_var[12] = tx;
	return_var[13] = ty;
	return_var[14] = tz;

	return_var
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
	let mut return_var = [0.; 16];

	return_var[0] = sx;
	return_var[5] = sy;
	return_var[10] = sz;
	return_var[15] = 1.;

	return_var
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
	let mut return_var = [0.; 16];

	return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
	return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
	return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
	return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

	return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
	return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
	return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
	return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

	return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
	return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
	return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
	return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

	return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
	return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
	return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
	return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

	return_var
}

pub fn invert_matrix(a: [f32; 16]) -> [f32;16] {
	let mut return_var = [0.; 16];

	let a00 = a[0];  let a01 = a[1];  let a02 = a[2]; let a03 = a[3];
	let a10 = a[4];  let a11 = a[5];  let a12 = a[6]; let a13 = a[7];
	let a20 = a[8];  let a21 = a[9];  let a22 = a[10]; let a23 = a[11];
	let a30 = a[12]; let a31 = a[13]; let a32 = a[14]; let a33 = a[15];

	let b00 = a00 * a11 - a01 * a10;
	let b01 = a00 * a12 - a02 * a10;
	let b02 = a00 * a13 - a03 * a10;
	let b03 = a01 * a12 - a02 * a11;
	let b04 = a01 * a13 - a03 * a11;
	let b05 = a02 * a13 - a03 * a12;
	let b06 = a20 * a31 - a21 * a30;
	let b07 = a20 * a32 - a22 * a30;
	let b08 = a20 * a33 - a23 * a30;
	let b09 = a21 * a32 - a22 * a31;
	let b10 = a21 * a33 - a23 * a31;
	let b11 = a22 * a33 - a23 * a32;

// Calculate the determinant
	let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

	if det == 0. {
		panic!("Determinant zero")
	}

	det = 1.0 / det;

	return_var[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
	return_var[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
	return_var[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
	return_var[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
	return_var[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
	return_var[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
	return_var[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
	return_var[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
	return_var[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
	return_var[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
	return_var[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
	return_var[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
	return_var[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
	return_var[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
	return_var[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
	return_var[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;

	return return_var;
}
