use glyph_brush::{GlyphBrushBuilder, FontId};
use glyph_brush::ab_glyph::FontArc;

pub struct Font {
    font_id: FontId,
}

pub const DEFAULT_FONT_SCALE: f32 = 16.0;

pub fn upload(data: Vec<u8>) {
    let font = FontArc::try_from_vec(data).unwrap();
    // let glyph_brush =
    GlyphBrushBuilder::using_font(font).build();
}
