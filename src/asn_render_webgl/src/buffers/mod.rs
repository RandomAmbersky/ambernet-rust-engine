use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use crate::GL;

pub fn load_buffer(gl: &GL, buf: &[f64]) -> WebGlBuffer {
	let memory_buffer = wasm_bindgen::memory()
		.dyn_into::<WebAssembly::Memory>()
		.unwrap()
		.buffer();
	let vertices_location = buf.as_ptr() as u32 / 4;
	let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
		vertices_location,
		vertices_location + buf.len() as u32,
	);

	let buffer = gl.create_buffer()
		.ok_or_else(||String::from("Failed to create buffer"))
		.unwrap();

	gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
	gl.buffer_data_with_array_buffer_view(
		GL::ARRAY_BUFFER,
		&vert_array,
		GL::STATIC_DRAW,
	);
	buffer
}
