use web_sys::WebGlTexture;
use amberskynet_logger_web::LoggerWeb;
pub use web_sys::WebGlRenderingContext as GL;
use asn_images::DecodedTexture;

#[allow(dead_code)]
pub fn update (
	gl: &GL,
	texture: &WebGlTexture,
	tex: &DecodedTexture,
	is_linear: bool
) -> Result<(), String>{

	gl.bind_texture(GL::TEXTURE_2D, Some(texture));

	// gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

	// CLAMP_TO_EDGE - from 0 to 1

	// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
	// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
	// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
	// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
	if is_linear {
		// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
		// gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
	} else {
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
	}

	let level = 0;
	let internal_format = GL::RGBA;
	let border = 0;
	let src_format = GL::RGBA;
	let src_type = GL::UNSIGNED_BYTE;

	match gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
		GL::TEXTURE_2D,
		level,
		internal_format as i32,
		tex.width,
		tex.height,
		border,
		src_format,
		src_type,
		Some(tex.bytes.as_slice()),
	) {
		Ok(t) => t,
		Err(why) => {
			let err_str = match why.as_string() {
				Some(t) => t,
				None => return Err(String::from("gl.tex_image_2d as_string error"))
			};
			let err = format!("gl.tex_image_2d error: {}", err_str);
			return Err(err)
		}
	};
	gl.bind_texture(GL::TEXTURE_2D, None);
	Ok(())
}

pub fn upload(
	gl: &GL,
	tex: &DecodedTexture,
	is_linear: bool
) -> Result<WebGlTexture, String> {
	let texture = match gl.create_texture() {
		Some(t) => t,
		None => {
			return Err(String::from("create_texture error"))
		}
	};
	update(gl, &texture,  &tex, is_linear)?;
	Ok(texture)
}
