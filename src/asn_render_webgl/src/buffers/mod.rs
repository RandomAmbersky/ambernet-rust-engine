use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use crate::GL;

pub fn load_buffer(gl: &GL, buf: &[f32]) -> WebGlBuffer {
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
	gl.bind_buffer(GL::ARRAY_BUFFER, None);
	buffer
}

pub fn load_index_buffer(gl: &GL, indices: &[u16]) -> WebGlBuffer {
	let memory_buffer = wasm_bindgen::memory()
		.dyn_into::<WebAssembly::Memory>()
		.unwrap()
		.buffer();
	let indices_location: u32 = indices.as_ptr() as u32 / 2;
	let indices_array = js_sys::Uint16Array::new(&memory_buffer).subarray(
		indices_location,
		indices_location + indices.len() as u32,
	);

	let indices_buffer = gl.create_buffer()
		.ok_or_else(||String::from("Failed to create buffer"))
		.unwrap();

	gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));
	gl.buffer_data_with_array_buffer_view(
		GL::ELEMENT_ARRAY_BUFFER,
		&indices_array,
		GL::STATIC_DRAW,
	);
	gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
	indices_buffer
}
