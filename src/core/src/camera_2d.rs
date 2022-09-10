use crate::{Array2D, Point2D, Size2D};

#[derive(Default, Debug)]
pub struct Camera2D {
	pub pos: Point2D,
	pub window: Size2D
}

impl Camera2D {
	pub fn look(&self, map: &Array2D, screen: &mut Array2D) -> Result<(), String> {

		// let mess = format!("src {:?} x {:?}", src.width, src.height);
		// LoggerWeb::log(&mess);

		if self.window.is_zero() {
			return Err(String::from("window size is zero"))
		}

		let mut bytes: Vec<u8> = Vec::new();

		let mut index = map.get_ingex(&self.pos)?;

		for _ in 0..self.window.height {
			let max_index = index + self.window.width as usize;
			// let mess = format!("set_view_from {:?} {:?} {:?}", window, index, max_index);
			// LoggerWeb::log(&mess);
			for c_x in index..max_index {
				bytes.push(map.bytes[c_x]);
				// let mess = format!("index {:?} {:?}", c_x, src.get_pos(c_x)?);
				// LoggerWeb::log(&mess);
			}
			index += map.width as usize;
		}

		screen.width = self.window.width;
		screen.height = self.window.height;
		screen.bytes = bytes;

		Ok(())
	}

	pub fn look_at(&mut self, map: &Array2D, screen: &mut Array2D) -> Result<(), String> {

		// let mess = format!("look_at: {:?} {:?}", pos, window);
		// LoggerWeb::log(&mess);

		let half_width = self.window.width / 2;
		let half_height = self.window.height / 2;

		let map_width_minus_width = map.width - self.window.width;
		let map_height_minus_height = map.height - self.window.height;

		// let mut n_pos: Point2D = self.pos;

		if self.pos.x > half_width {
			self.pos.x -= half_width;
		} else {
			self.pos.x = 0;
		}

		if self.pos.y > half_height {
			self.pos.y -= half_height;
		} else {
			self.pos.y = 0;
		}

		if self.pos.y > map_height_minus_height {
			self.pos.y = map_height_minus_height;
		}

		if self.pos.x > map_width_minus_width {
			self.pos.x = map_width_minus_width;
		}

		// let mess = format!("n_pos: {:?}", n_pos);
		// LoggerWeb::log(&mess);

		self.look(map, screen)
	}
}
