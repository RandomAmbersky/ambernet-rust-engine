use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};
use asn_core::{Array2D, Size2D};
use asn_render_webgl::RenderContext;
use crate::{GL, utils};

pub struct RenderTiles {
    texture: WebGlTexture,
    map_texture: WebGlTexture,
    program: WebGlProgram,
    a_position: u32,
    u_transform: WebGlUniformLocation,
    transform_matrix: [f32;16],
    u_image0: WebGlUniformLocation,
    u_image1: WebGlUniformLocation,
    vertices_buf: WebGlBuffer,
    u_map_size: WebGlUniformLocation,
    u_sheet_size: WebGlUniformLocation,
    u_tile_size: WebGlUniformLocation,
}

impl RenderTiles {
    pub fn new (ctx: &RenderContext) -> Result<RenderTiles, String> {
        let vertices_buf = asn_render_webgl::load_buffer(ctx, &utils::VERTICES);

        let program = match asn_render_webgl::link_program(ctx, utils::VERTEX_SHADER, utils::FRAG_SHADER) {
            Ok(t) => t,
            Err(str) => {
                return Err(str)
            }
        };

        let texture = asn_render_webgl::load_empty_texture(ctx)?;
        let map_texture = asn_render_webgl::load_empty_texture(ctx)?;

        let a_position = ctx.gl.get_attrib_location(&program, "aPosition") as u32;

        let u_transform =  ctx.gl.get_uniform_location(&program, "uTransform").unwrap();

        let scale_matrix = asn_core::math::scaling_matrix(
            2.,
            2.,
            1.
        );
        let trans_matrix = asn_core::math::translation_matrix(
            -1.,
            -1.,
            0.
        );

        let transform_matrix = asn_core::math::mult_matrix_4(scale_matrix, trans_matrix);

        let u_image0 = match ctx.gl.get_uniform_location(&program, "uTileMap") {
            Some(t) => t,
            None => {
                return Err(String::from("uTileMap not found"))
            }
        };

        let u_image1 =  match ctx.gl.get_uniform_location(&program, "uTileSheet") {
            Some(t) => t,
            None => {
                return Err(String::from("uTileSheet not found"))
            }
        };

        let u_map_size = match ctx.gl.get_uniform_location(&program, "uMapSize") {
            None => {
                return Err(String::from("uMapSize not found"))
            },
            Some(t) => t
        };

        let u_sheet_size = match ctx.gl.get_uniform_location(&program, "uSheetSize") {
            None => {
                return Err(String::from("uSheetSize not found"))
            },
            Some(t) => t
        };

        let u_tile_size = match ctx.gl.get_uniform_location(&program, "utileSize") {
            None => {
                return Err(String::from("uSheetSize not found"))
            },
            Some(t) => t
        };

        let data = RenderTiles {
            program,
            a_position,
            texture,
            map_texture,
            u_transform,
            transform_matrix,
            u_image0,
            u_image1,
            vertices_buf,
            u_map_size,
            u_sheet_size,
            u_tile_size
        };
        Ok(data)
    }

    pub fn set_tiles (&self, ctx: &RenderContext, tex: &Array2D, tile_size: &Size2D) -> Result<(), String> {
        asn_render_webgl::update_texture(ctx, &self.texture, tex, false)?;
        ctx.gl.use_program(Some(&self.program));
        ctx.gl.uniform2f(Some(&self.u_sheet_size), tex.size.width as f32, tex.size.height as f32);
        ctx.gl.uniform2f(Some(&self.u_tile_size), tile_size.width as f32, tile_size.height as f32);
        ctx.gl.use_program(None);
        Ok(())
    }
    
    pub fn update_view (&self, ctx: &RenderContext, texture_data: &Array2D) -> Result<(), String> {
        asn_render_webgl::update_texture(ctx, &self.map_texture, texture_data, false)?;
        ctx.gl.use_program(Some(&self.program));
        ctx.gl.uniform2f(Some(&self.u_map_size), texture_data.size.width as f32, texture_data.size.height as f32);
        ctx.gl.use_program(None);
        Ok(())
    }
    
    pub fn draw(&self, ctx: &RenderContext) {
        ctx.gl.use_program(Some(&self.program));

        ctx.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &self.transform_matrix);

        ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&self.vertices_buf));
        ctx.gl.vertex_attrib_pointer_with_i32(self.a_position, 2, GL::FLOAT, false, 0, 0);
        ctx.gl.enable_vertex_attrib_array(self.a_position);
        ctx.gl.bind_buffer( GL::ARRAY_BUFFER, None);

        ctx.gl.active_texture(GL::TEXTURE0);
        ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&self.map_texture));
        ctx.gl.uniform1i(Some(&self.u_image0), 0);

        ctx.gl.active_texture(GL::TEXTURE1);
        ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        ctx.gl.uniform1i(Some(&self.u_image1), 1);

        ctx.gl.draw_arrays(GL::TRIANGLES, 0, 6);

        ctx.gl.active_texture(GL::TEXTURE0);
        ctx.gl.bind_texture(GL::TEXTURE_2D, None);

        ctx.gl.active_texture(GL::TEXTURE1);
        ctx.gl.bind_texture(GL::TEXTURE_2D, None);
    }
}
