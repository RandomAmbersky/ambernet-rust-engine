use crate::core::Delta2D;

#[derive(Default, Debug, Clone, Copy)]
pub struct Point2dU32 {
    pub x: u32,
    pub y: u32,
}

impl Point2dU32 {
    pub fn apply(&mut self, delta: &Delta2D) -> Result<(), String> {
        if self.x == 0 && delta.x < 0 {
            return Err(String::from("overflow -x"));
        }
        if self.y == 0 && delta.y < 0 {
            return Err(String::from("overflow -y"));
        }
        self.x += delta.x as u32;
        self.y += delta.y as u32;
        Ok(())
    }

    pub fn add(&self, delta: &Delta2D) -> Result<Point2dU32, String> {
        let mut new_pos = *self;
        new_pos.apply(delta)?;
        Ok(new_pos)
    }
}
