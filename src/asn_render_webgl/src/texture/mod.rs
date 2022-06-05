use web_sys::WebGlTexture;
use crate::GL;

#[allow(dead_code)]
pub struct Texture {
	texture: Option<WebGlTexture>
	// width: img.width(),
	// height: img.height()
}

#[allow(dead_code)]
pub fn upload_texture(_gl: &GL, _bytes: &[u8]) -> Texture {
	// let img = match image::load_from_memory(bytes) {
	// 	Ok(t) => t,
	// 	Err(why) => {
	// 		panic!("image::load_from_memory error: {}", why)
	// 	}
	// };
	// let decode_bytes = img.to_rgba8().into_raw();
	// let tex = match gl.create_texture() {
	// 	Some(t) => t,
	// 	None => {
	// 		panic!("create_texture error")
	// 	}
	// };
	// gl.bind_texture(GL::TEXTURE_2D, Some(tex.as_ref()));
	// gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

	// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
	// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
	// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
	// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
	// ctx.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
	// ctx.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

	// target: u32,
	// level: i32,
	// internalformat: i32,
	// width: i32,
	// height: i32,
	// border: i32,
	// format: u32,
	// type_: u32,
	// pixels: Option<&[u8]>,

	// let level = 0;
	// let internal_format = GL::RGBA;
	// let border = 0;
	// let src_format = GL::RGBA;
	// let src_type = GL::UNSIGNED_BYTE;

	// let load_result = match gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
	// 	GL::TEXTURE_2D,
	// 	level,
	// 	internal_format as i32,
	// 	img.width() as i32,
	// 	img.height() as i32,
	// 	border,
	// 	src_format,
	// 	src_type,
	// 	Some(&decode_bytes),
	// ) {
	// 	Ok(t) => t,
	// 	Err(why) => {
	// 		let err_str = match why.as_string() {
	// 			Some(t) => t,
	// 			None => panic!("gl.tex_image_2d as_string error")
	// 		};
	// 		panic!("gl.tex_image_2d error: {}", &err_str);
	// 	}
	// };

	Texture {
		texture: None,
		// width: img.width(),
		// height: img.height()
	}
}
