use web_sys::{WebGlProgram, WebGlBuffer, WebGlUniformLocation};
use web_sys::WebGlRenderingContext as GL;

use crate::amberskynet::math::matrix as M;
use crate::amberskynet::render::RenderContext;

pub struct Test2D {
    pub program: WebGlProgram,
    pub buffer: WebGlBuffer,
    pub u_color: WebGlUniformLocation,
    pub u_opacity: WebGlUniformLocation,
    pub u_transform: WebGlUniformLocation,
}

impl Test2D {
    pub fn render(&self, ctx: &RenderContext) {
        ctx.gl.use_program(Some(&self.program));

        let bottom: f32 = 1.;
        let top: f32 = 1.;
        let left: f32 = 1.;
        let right: f32 = 1.;
        let canvas_height: f32 = 1.;
        let canvas_width: f32 = 1.;

        ctx.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        ctx.gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        ctx.gl.enable_vertex_attrib_array(0);

        ctx.gl.uniform4f(
            Some(&self.u_color),
            0., //r
            0.5,//g
            0.5,//b
            1.0,//a
        );

        ctx.gl.uniform1f(Some(&self.u_opacity), 1.);

        let translation_mat = M::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.,
        );

        let scale_mat = M::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.,
        );

        let transform_mat = M::mult_matrix_4(scale_mat, translation_mat);
        ctx.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

        let rect_vertice_ary_length = 12;
        ctx.gl.draw_arrays(GL::TRIANGLES, 0, (rect_vertice_ary_length / 2) as i32);
        // log("OK");
    }
}

// glyph_brush -?
