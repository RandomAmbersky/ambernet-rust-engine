use web_sys::{WebGlRenderingContext as GL};
use web_sys::{WebGlBuffer, WebGlProgram, WebGlUniformLocation};
use asn_math::{mult_matrix_4, scaling_matrix, translation_matrix};
use asn_render_webgl::{link_program, load_buffer, RenderContext};

pub struct Test2D {
	pub program: WebGlProgram,
	pub buffer: WebGlBuffer,
	pub u_color: WebGlUniformLocation,
	pub u_opacity: WebGlUniformLocation,
	pub u_transform: WebGlUniformLocation,
}

pub fn new_item (
	ctx: &RenderContext,
	vert: &str,
	frag: &str,
	mesh: &[f32]
) -> Test2D {
	let gl = &ctx.gl;
		let program = link_program(ctx, vert, frag).unwrap();
		let buf = load_buffer(ctx, mesh);
		Test2D {
			u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
			u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
			u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
			buffer: buf,
			program,
		}
}

pub fn render (ctx: &RenderContext, item: &Test2D) {

	let gl = &ctx.gl;

	gl.use_program(Some(&item.program));

	let bottom: f32 = 1.;
	let top: f32 = 1.;
	let left: f32 = 1.;
	let right: f32 = 1.;
	let canvas_height: f32 = 1.;
	let canvas_width: f32 = 1.;

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
