use asn_render_webgl::RenderContext;

pub struct View2D {
	w: i32,
	h: i32
}

pub fn new_item (w: i32, h: i32) -> View2D {
	View2D {w, h}
}

pub fn draw (_: &RenderContext, _: &View2D) {

}
