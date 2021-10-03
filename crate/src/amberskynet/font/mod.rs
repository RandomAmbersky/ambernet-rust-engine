use ab_glyph::{Glyph, point, FontArc, Font};

pub struct Text {
    font: &'static FontArc,
    glyph: Glyph
}

pub const DEFAULT_FONT_SCALE: f32 = 16.0;

pub fn upload(data: Vec<u8>) -> FontArc {
    let font = FontArc::try_from_vec(data).unwrap();
    // let glyph: Glyph = font.glyph_id('q').with_scale_and_position(
    //     24.0,
    //     point(100.0, 0.0)
    // );
    // let text = Text {
    //     font: &font,
    //     glyph
    // };
    font
}

impl Text {
    pub fn new(font: &'static FontArc) -> Self {
        let glyph: Glyph = font.glyph_id('q').with_scale_and_position(
            24.0,
            point(100.0, 0.0)
        );
        Self {
            font,
            glyph
        }
    }
    pub fn draw (&self) {
        let glyph: Glyph = self.font.glyph_id('q').with_scale_and_position(
            24.0,
            point(100.0, 0.0)
        );
        if let Some(glyph) = self.font.outline_glyph(glyph) {
            glyph.draw( |x, y, c| { /* draw pixel `(x, y)` with coverage: `c` */

            });
        }
    }
}
