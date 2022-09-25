use asn_core::math::{mult_matrix_4, scaling_matrix, translation_matrix};
use asn_render_webgl::{GL, link_program, load_buffer, RenderBuffer, RenderContext, RenderProgram, RenderUniformLocation};

pub struct Test2D {
	pub program: RenderProgram,
	pub buffer: RenderBuffer,
	pub u_color: RenderUniformLocation,
	pub u_opacity: RenderUniformLocation,
	pub u_transform: RenderUniformLocation,
}

pub fn new_item (
	ctx: &RenderContext,
	vert: &str,
	frag: &str,
	mesh: &[f32]
) -> Result<Test2D, String> {
	let gl = &ctx.gl;
		let program = link_program(ctx, vert, frag)?;
		let buf = load_buffer(ctx, mesh);
		let test_2d = Test2D {
			u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
			u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
			u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
			buffer: buf,
			program,
		};
	Ok(test_2d)
}

pub fn render (ctx: &RenderContext, item: &Test2D) {

	let gl = &ctx.gl;

	gl.use_program(Some(&item.program));

	let bottom: f32 = 10.;
	let top: f32 = 100.;
	let left: f32 = 10.;
	let right: f32 = 100.;
	let canvas_height: f32 = ctx.width as f32;
	let canvas_width: f32 = ctx.height as f32;

	gl.bind_buffer(GL::ARRAY_BUFFER, Some(&item.buffer));
	gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
	gl.enable_vertex_attrib_array(0);

	gl.uniform4f(
		Some(&item.u_color),
		0., //r
		0.5,//g
		0.5,//b
		1.0,//a
	);

	gl.uniform1f(Some(&item.u_opacity), 1.);

	let translation_mat = translation_matrix(
		2. * left / canvas_width - 1.,
		2. * bottom / canvas_height - 1.,
		0.,
	);

	let scale_mat = scaling_matrix(
		2. * (right - left) / canvas_width,
		2. * (top - bottom) / canvas_height,
		0.,
	);

	let transform_mat = mult_matrix_4(scale_mat, translation_mat);
	gl.uniform_matrix4fv_with_f32_array(Some(&item.u_transform), false, &transform_mat);

	let rect_vertice_ary_length = 12;
	gl.draw_arrays(GL::TRIANGLES, 0, (rect_vertice_ary_length / 2) as i32);
}
